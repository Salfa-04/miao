//!
//! # System Interrupts
//!

use super::private::*;

bind_interrupts! {
    pub struct Irqs {
        // LPUART1 => hal::usart::InterruptHandler<peripherals::LPUART1>;
        // FDCAN1_IT0 => hal::can::IT0InterruptHandler<peripherals::FDCAN1>;
        // FDCAN1_IT1 => hal::can::IT1InterruptHandler<peripherals::FDCAN1>;
    }
}
