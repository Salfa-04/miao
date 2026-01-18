#![no_std]
#![no_main]

use utils::prelude::*;

mod controller;
mod system;

mod tasks {
    pub mod blinky;
    pub mod health;
}

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (mut c, p) = utils::sys_init();
    let r = {
        use system::*;
        split_resources!(p)
    };

    {
        use utils::peripheral::SCB;
        let scb = &mut c.SCB;
        let _ = scb.enable_icache();
        let i = SCB::icache_enabled();
        let d = SCB::dcache_enabled();
        defmt::trace!("ICache: {}, DCache: {}", i, d);
    }

    s.must_spawn(tasks::health::task());

    s.must_spawn(tasks::blinky::task(r.blinky));

    s.must_spawn(controller::main());

    {
        use dji_frame::*;

        struct GameStatus {
            game_type: u8,
            game_progress: u8,
            stage_remain_time: u16,
            sync_timestamp: u64,
        }

        impl Marshaler for GameStatus {
            const CMD_ID: u16 = 0x0001;

            fn marshal(&self, buf: &mut [u8]) -> Result<usize> {
                let mut cursor = 0;

                buf[cursor] = (self.game_type & 0xF) | (self.game_progress & 0xF) << 4;
                cursor += 1;

                buf[cursor..cursor + 2].copy_from_slice(&self.stage_remain_time.to_le_bytes());
                cursor += 2;

                buf[cursor..cursor + 8].copy_from_slice(&self.sync_timestamp.to_le_bytes());
                cursor += 8;

                Ok(cursor)
            }

            fn unmarshal(_: &[u8]) -> Result<Self> {
                unimplemented!()
            }
        }

        let mut msger: Messager<DjiValidator> = Messager::new(0x13);

        #[unsafe(link_section = ".axisram.uart1.tx")]
        static BUF: utils::MemCell<[u8; 64]> = utils::MemCell::uninit();

        let buf = unsafe { &mut *BUF.init([0; _]) };

        let p = r.uart1;
        let mut config = hal::usart::Config::default();
        config.baudrate = 115200;
        config.parity = hal::usart::Parity::ParityNone;
        config.stop_bits = hal::usart::StopBits::STOP1;
        config.data_bits = hal::usart::DataBits::DataBits8;

        let mut uart = hal::usart::UartTx::new(p.usart_p, p.usart_tx, p.dma_tx, config).unwrap();

        loop {
            let game = GameStatus {
                game_type: 1,
                game_progress: 2,
                stage_remain_time: 300,
                sync_timestamp: time::Instant::now().as_millis(),
            };

            let size = msger.pack(&game, buf).unwrap();
            let game = &mut buf[..size];

            game[size - 1] = 0;
            game[size - 2] = 0;

            let s = uart.write(game).await;
            defmt::trace!("UART: {}", s);
            utils::T::after_millis(300).await;
        }
    }
}
