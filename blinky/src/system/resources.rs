//!
//! # System Resources
//!
//! ## Reserved Resources
//!

use super::private::*;

assign_resources! {
    /// for `Blinky` task.
    blinky: BlinkySrc {
        spi_p: SPI6,
        led_pin: PA7,
        dma: BDMA_CH0,
    }
}
