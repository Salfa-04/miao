use crate::{ef::select, hal, system::*};
use gpio::{Level, Output as OP, Pull, Speed};
use hal::{exti::ExtiInput, gpio, mode::Async, spi, time::mhz};
use select::{Either, select};
use spi::{BitOrder, Config, MODE_3, Spi};

const WAIT_IV: u64 = 150; // us
const WAIT_RESET: u64 = 50; // ms

#[derive(defmt::Format, Debug)]
pub enum WaitInt {
    Gyro,
    Acc,
}

pub struct BMI088<'t> {
    imu: Spi<'t, Async>,
    acc_cs: OP<'t>,
    acc_int: ExtiInput<'t>,
    gyro_cs: OP<'t>,
    gyro_int: ExtiInput<'t>,
    buffer: &'static mut [u8],
}

impl BMI088<'_> {
    pub fn new(p: ImuSrc, buffer: &'static mut [u8]) -> Self {
        if buffer.len() < 8 {
            panic!("BMI088 Buffer Size MUST be at Least 8 Bytes");
        }

        let acc_int = ExtiInput::new(p.acc_int, p.acc_exti, Pull::Up);
        let gyro_int = ExtiInput::new(p.gyro_int, p.gyro_exti, Pull::Up);
        let acc_cs = OP::new(p.acc_cs, Level::High, Speed::VeryHigh);
        let gyro_cs = OP::new(p.gyro_cs, Level::High, Speed::VeryHigh);

        let mut config = Config::default();
        config.mode = MODE_3; // HIGH, 2EDGE
        config.bit_order = BitOrder::MsbFirst;
        config.frequency = mhz(10);
        config.miso_pull = Pull::Up;
        config.gpio_speed = Speed::Medium;

        let imu = Spi::new(
            p.spi_p, p.spi_sck, p.spi_mosi, p.spi_miso, p.dma_tx, p.dma_rx, config,
        );

        Self {
            imu,
            acc_cs,
            acc_int,
            gyro_cs,
            gyro_int,
            buffer,
        }
    }
}

impl BMI088<'_> {
    pub async fn init(&mut self) -> bool {
        let gyro = self.init_gyro().await;
        let acc = self.init_acc().await;
        gyro && acc // OK for true
    }

    #[inline]
    pub fn wait_new_gyro(&mut self) -> impl Future<Output = ()> {
        self.gyro_int.wait_for_falling_edge()
    }

    #[inline]
    pub fn wait_new_acc(&mut self) -> impl Future<Output = ()> {
        self.acc_int.wait_for_falling_edge()
    }

    pub async fn wait(&mut self) -> WaitInt {
        let acc = self.acc_int.wait_for_falling_edge();
        let gyro = self.gyro_int.wait_for_falling_edge();
        let result = select(acc, gyro).await;

        match result {
            Either::First(_) => WaitInt::Acc,
            Either::Second(_) => WaitInt::Gyro,
        }
    }
}

impl BMI088<'_> {
    pub async fn read_gyro(&mut self) -> (f64, f64, f64) {
        #[unsafe(link_section = ".axisram.imu")]
        static READ_GYRO: [u8; 7] = [
            // 0x82: 0x02 | 0x80, Read 6 bytes from 0x02
            0x82, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        self.acc_cs.set_high();
        let buf = &mut self.buffer[..READ_GYRO.len()];
        self.gyro_cs.set_low();
        let _ = self.imu.transfer(buf, &READ_GYRO).await;
        self.gyro_cs.set_high();

        /// Note: `16.384` is Determined
        /// by Register `GYRO_RANGE(0x0F)`(±2000dps)
        const PREF: f64 = 1. / 16.384; // dps/LSB
        (
            (i16::from_le_bytes([buf[1], buf[2]]) as f64 * PREF),
            (i16::from_le_bytes([buf[3], buf[4]]) as f64 * PREF),
            (i16::from_le_bytes([buf[5], buf[6]]) as f64 * PREF),
        ) // Return in dps
    }

    pub async fn read_acc(&mut self) -> (f64, f64, f64) {
        #[unsafe(link_section = ".axisram.imu")]
        static READ_ACC: [u8; 8] = [
            // 0x92: 0x12 | 0x80, Read 6 bytes from 0x12
            0x92, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        self.gyro_cs.set_high();
        let buf = &mut self.buffer[..READ_ACC.len()];
        self.acc_cs.set_low();
        let _ = self.imu.transfer(buf, &READ_ACC).await;
        self.acc_cs.set_high();

        /// Note: `12.` is Determined
        /// by Register `ACC_RANGE(0x41)`(±12g)
        const PREF: f64 = 1. / 32768. * 1000. * 12.; // mg/LSB
        (
            (i16::from_le_bytes([buf[2], buf[3]]) as f64 * PREF),
            (i16::from_le_bytes([buf[4], buf[5]]) as f64 * PREF),
            (i16::from_le_bytes([buf[6], buf[7]]) as f64 * PREF),
        ) // Return in mg
    }

    /// The temperature sensor data is updated every 1.28s
    pub async fn read_temp(&mut self) -> f32 {
        #[unsafe(link_section = ".axisram.imu")]
        static READ_TEMP: [u8; 4] = [
            // 0xA2: 0x22 | 0x80, Read 2 bytes from 0x22
            0xA2, 0xFF, 0xFF, 0xFF,
        ];

        self.gyro_cs.set_high();
        let buf = &mut self.buffer[..READ_TEMP.len()];
        self.acc_cs.set_low();
        let _ = self.imu.transfer(buf, &READ_TEMP).await;
        self.acc_cs.set_high();

        let temp = ((buf[2] as i16) << 3) | ((buf[3] as i16) >> 5);
        let temp = if temp > 0x3FF { temp - 0x800 } else { temp };
        temp as f32 * 0.125 + 23. // in °C
    }
}

impl BMI088<'_> {
    async fn init_gyro(&mut self) -> bool {
        // Read GYRO_CHIP_ID: 0x0F
        self.read_reg_gyro(0x00).await;
        utils::T::after_micros(WAIT_IV).await;
        // GYRO_SOFTRESET: Write 0xB6 to Reset
        self.write_reg_gyro(0x14, 0xB6).await;
        utils::T::after_millis(WAIT_RESET).await;
        // Read GYRO_CHIP_ID: 0x0F
        self.read_reg_gyro(0x00).await;
        utils::T::after_micros(WAIT_IV).await;
        if self.read_reg_gyro(0x00).await != 0xF {
            return false;
        }

        for (reg, val) in [
            (0x0F, 0x00), // GYRO_RANGE: ±2000dps
            (0x10, 0x82), // GYRO_BANDWIDTH: ODR=1000Hz, FBW=116Hz
            (0x11, 0x00), // GYRO_LPM1: Normal mode
            (0x15, 0x80), // GYRO_INT_CTRL: New Data Interrupt
            (0x16, 0x02), // INT3_INT4_IO_CONF: Open-Drain, Active Low
            (0x18, 0x01), // INT3_INT4_IO_MAP: Data Ready Interrupt to INT3
        ] {
            utils::T::after_micros(WAIT_IV).await;
            self.write_reg_gyro(reg, val).await;
            utils::T::after_micros(WAIT_IV).await;
            if self.read_reg_gyro(reg).await != val {
                return false;
            }

            // let v = self.read_reg_gyro(reg).await;
            // defmt::info!("BMI088 GYRO REG {}({}): {}", reg, val, v);
        }

        true
    }

    async fn init_acc(&mut self) -> bool {
        // Read ACC_CHIP_ID: 0x1E
        self.read_reg_acc(0x00).await;
        utils::T::after_micros(WAIT_IV).await;
        // ACC_SOFTRESET: Write 0xB6 to Reset
        self.write_reg_acc(0x7E, 0xB6).await;
        utils::T::after_millis(WAIT_RESET).await;
        // Read ACC_CHIP_ID: 0x1E
        self.read_reg_acc(0x00).await;
        utils::T::after_micros(WAIT_IV).await;
        if self.read_reg_acc(0x00).await != 0x1E {
            return false;
        }

        for (reg, val) in [
            (0x7D, 0x04), // ACC_PWR_CTRL: Enable Accelerometer
            (0x7C, 0x00), // ACC_PWR_CONF: Active Mode
            (0x40, 0xA9), // ACC_CONF: ODR=200Hz, OSR=1x
            (0x41, 0x02), // ACC_RANGE: ±12g
            (0x53, 0x0C), // INT1_IO_CONF: Output, Open-Drain, Active Low
            (0x58, 0x04), // INT1_INT2_MAP_DATA: Data Ready Interrupt to INT1
        ] {
            utils::T::after_micros(WAIT_IV).await;
            self.write_reg_acc(reg, val).await;
            utils::T::after_micros(WAIT_IV).await;
            if self.read_reg_acc(reg).await != val {
                return false;
            }

            // let v = self.read_reg_acc(reg).await;
            // defmt::info!("BMI088 ACC REG {}({}): {}", reg, val, v);
        }

        true
    }
}

impl BMI088<'_> {
    async fn read_reg_gyro(&mut self, reg: u8) -> u8 {
        self.acc_cs.set_high();
        let buf = &mut self.buffer[..2];
        buf[0] = reg | 0x80;
        buf[1] = 0xFF;

        self.gyro_cs.set_low();
        let _ = self.imu.transfer_in_place(buf).await;
        self.gyro_cs.set_high();
        buf[1]
    }

    async fn write_reg_gyro(&mut self, reg: u8, val: u8) {
        self.acc_cs.set_high();
        let buf = &mut self.buffer[..2];
        buf[0] = reg & 0x7F;
        buf[1] = val;

        self.gyro_cs.set_low();
        let _ = self.imu.write(buf).await;
        self.gyro_cs.set_high();
    }

    async fn read_reg_acc(&mut self, reg: u8) -> u8 {
        self.gyro_cs.set_high();
        let buf = &mut self.buffer[..3];
        buf[0] = reg | 0x80;
        buf[1] = 0xFF;
        buf[2] = 0xFF;

        self.acc_cs.set_low();
        let _ = self.imu.transfer_in_place(buf).await;
        self.acc_cs.set_high();
        buf[2]
    }

    async fn write_reg_acc(&mut self, reg: u8, val: u8) {
        self.gyro_cs.set_high();
        let buf = &mut self.buffer[..2];
        buf[0] = reg & 0x7F;
        buf[1] = val;

        self.acc_cs.set_low();
        let _ = self.imu.write(buf).await;
        self.acc_cs.set_high();
    }
}
