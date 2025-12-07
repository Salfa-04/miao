//!
//! # System Resources
//!
//! ## Reserved Resources
//!

use super::private::*;

assign_resources! {
    /// for `Blinky` task.
    blinky: BlinkySrc {
        led_pin: PC13,
    }
}
