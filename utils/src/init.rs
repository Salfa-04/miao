//!
//! # System Initialization
//!

use crate::prelude::{hal, ll};
use hal::{Config, Peripherals, init, rcc, time::mhz};
use ll::{Peripherals as CorePeripherals, peripheral::SCB};

// __pre_init function to be called before main
core::arch::global_asm! {
    ".global __pre_init",
    ".type __pre_init, %function",
    ".thumb_func",
    "__pre_init:",

    // Copy ITCM from FLASH to ITCM RAM
    "ldr r0, =__sitcm
     ldr r1, =__eitcm
     ldr r2, =__siitcm
     0:
     cmp r1, r0
     beq 1f
     ldm r2!, {{r3, r4}}
     stm r0!, {{r3, r4}}
     b 0b
     1:",

    // Copy AXISRAM from FLASH to AXISRAM RAM
    "ldr r0, =__saxisram
     ldr r1, =__eaxisram
     ldr r2, =__siaxisram
     0:
     cmp r1, r0
     beq 1f
     ldm r2!, {{r3, r4}}
     stm r0!, {{r3, r4}}
     b 0b
     1:",

    "bx lr", // Return from __pre_init
}

///
/// # System Initialization Function
///
/// This function initializes the system peripherals and clocks.
///
pub fn sys_init() -> (CorePeripherals, Peripherals) {
    defmt::debug!("System Initialization...");

    let core = match CorePeripherals::take() {
        None => panic!("{}: Can Be Called Only Once!!!", file!()),
        Some(mut x) => {
            x.SCB.enable_icache();
            let i = SCB::icache_enabled();
            let d = SCB::dcache_enabled();
            defmt::trace!("icache: {}, dcache: {}", i, d);
            x
        }
    };

    let peripherals = {
        let mut config = Config::default();
        config.enable_debug_during_sleep = true;

        let rcc = &mut config.rcc;

        rcc.hsi = None; // HSI = 64MHz
        rcc.hse = Some(rcc::Hse {
            freq: mhz(24), // HSE = 24MHz
            mode: rcc::HseMode::Oscillator,
        });

        rcc.csi = false; // CSI = 4MHz
        rcc.hsi48 = None; // HSI48 = 48Mhz

        rcc.pll1 = Some(rcc::Pll {
            source: rcc::PllSource::HSE,   //  24Mhz
            prediv: rcc::PllPreDiv::DIV3,  //   8Mhz
            mul: rcc::PllMul::MUL65,       // 520Mhz
            divp: Some(rcc::PllDiv::DIV1), // 520Mhz
            divq: Some(rcc::PllDiv::DIV4), // 130Mhz
            divr: None,                    //
        });

        rcc.pll2 = None; // Disabled

        rcc.pll3 = Some(rcc::Pll {
            source: rcc::PllSource::HSE,    //  24Mhz
            prediv: rcc::PllPreDiv::DIV4,   //   6Mhz
            mul: rcc::PllMul::MUL125,       // 750Mhz
            divp: Some(rcc::PllDiv::DIV10), //  75Mhz
            divq: Some(rcc::PllDiv::DIV6),  // 125Mhz
            divr: Some(rcc::PllDiv::DIV5),  // 150Mhz
        });

        rcc.sys = rcc::Sysclk::PLL1_P; // 520Mhz
        rcc.d1c_pre = rcc::AHBPrescaler::DIV1; // 520Mhz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV2; // 260Mhz
        rcc.apb1_pre = rcc::APBPrescaler::DIV2; // 130Mhz
        rcc.apb2_pre = rcc::APBPrescaler::DIV2; // 130Mhz
        rcc.apb3_pre = rcc::APBPrescaler::DIV2; // 130Mhz
        rcc.apb4_pre = rcc::APBPrescaler::DIV2; // 130Mhz

        rcc.timer_prescaler = rcc::TimerPrescaler::DefaultX2; // 260MHZ
        rcc.voltage_scale = rcc::VoltageScale::Scale0; // Max Performance
        rcc.ls = rcc::LsConfig::default_lsi(); // LSI = 32KHz
        rcc.supply_config = rcc::SupplyConfig::SMPSLDO(
            // SMPS to LDO with 1.8V output
            rcc::SMPSSupplyVoltage::V1_8,
        );

        let mux = &mut rcc.mux;
        mux.spi123sel = rcc::mux::Saisel::PLL3_P; // 75Mhz
        mux.usart234578sel = rcc::mux::Usart234578sel::PLL3_Q; // 125Mhz
        mux.usart16910sel = rcc::mux::Usart16910sel::PLL3_Q; // 125Mhz
        mux.rngsel = rcc::mux::Rngsel::PLL1_Q; // 130Mhz
        mux.spi6sel = rcc::mux::Spi6sel::HSE; // 24Mhz
        mux.octospisel = rcc::mux::Fmcsel::HCLK3; // 260MHz
        mux.adcsel = rcc::mux::Adcsel::PLL3_R; // 150Mhz
        mux.fdcansel = rcc::mux::Fdcansel::PLL1_Q; // 130Mhz
        mux.usbsel = rcc::mux::Usbsel::PLL3_Q; // 125Mhz

        init(config) // SysClock = 520MHz
    };

    (core, peripherals)
}
