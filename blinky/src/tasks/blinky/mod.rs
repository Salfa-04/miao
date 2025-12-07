//!
//! # Blinky Task
//!

use crate::{hal, system::*};
use hal::gpio::{Pull, Speed};
use hal::spi::{BitOrder, Config, MODE_0, Spi};
use hal::time::khz;

const SPEED: f32 = 0.3;

#[embassy_executor::task]
pub async fn task(p: BlinkySrc) -> ! {
    let mut t = utils::init_ticker!(1);

    let mut config = Config::default();
    config.mode = MODE_0;
    config.bit_order = BitOrder::LsbFirst;
    config.frequency = khz(6400);
    config.miso_pull = Pull::None;
    config.gpio_speed = Speed::Medium;

    let mut led = Spi::new_txonly_nosck(p.spi_p, p.led_pin, p.dma, config);

    let mut hue = 0.;

    loop {
        let (r, g, b) = color_wheel(hue as _);
        hue = (hue + SPEED) % 1536.;

        let buf = ws2812_calc(r, g, b);
        let _ = led.write(buf).await;

        t.next().await
    }
}

/// # Calculate WS2812 Data Buffer
/// Prepares the data buffer for WS2812 LED based on RGB values.
fn ws2812_calc<'t>(r: u8, g: u8, b: u8) -> &'t [u8] {
    /// Safety: **Only called in Single-Threaded Context.**
    #[unsafe(link_section = ".sram4.blinky")]
    static mut BUFFER: [u8; 25] = [0; _];
    let buf = &raw mut BUFFER;

    const N0: u8 = 0b1110_0000; // bit 0
    const N1: u8 = 0b1111_1000; // bit 1

    let mut temp = [0; _];
    for i in 0..8 {
        temp[i + 0] = if (g << i) & 0x80 != 0 { N1 } else { N0 };
        temp[i + 8] = if (r << i) & 0x80 != 0 { N1 } else { N0 };
        temp[i + 16] = if (b << i) & 0x80 != 0 { N1 } else { N0 };
    }

    // Safety: We have a valid pointer to BUFFER here
    unsafe {
        buf.write(temp);
        buf.as_ref().unwrap()
    }
}

/// # HUE to RGB Conversion
/// Converts a hue value (0-1535) to RGB values (0-255).
const fn color_wheel(hue: u16) -> (u8, u8, u8) {
    let x = (hue & 0xFF) as u8;
    match hue >> 8 {
        0 => (255, x, 0),       // Red -> Yellow
        1 => (255 - x, 255, 0), // Yellow -> Green
        2 => (0, 255, x),       // Green -> Cyan
        3 => (0, 255 - x, 255), // Cyan -> Blue
        4 => (x, 0, 255),       // Blue -> Magenta
        _ => (255, 0, 255 - x), // Magenta -> Red
    }
}
