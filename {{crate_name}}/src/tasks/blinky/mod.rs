//!
//! # Blinky Task
//!

use crate::{hal, system::*};
use hal::gpio::{Level, Output as OP, Speed};

#[embassy_executor::task]
pub async fn task(p: BlinkySrc) -> ! {
    let mut t = utils::init_ticker!(150); // 150ms

    let mut led = OP::new(p.led_pin, Level::Low, Speed::Low);

    loop {
        led.toggle();
        t.next().await
    }
}
