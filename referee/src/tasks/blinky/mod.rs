//!
//! # Blinky Task
//!

use crate::{hal, system::*};
use gpio::{Pull, Speed};
use hal::{gpio, spi, time::mhz};
use spi::{BitOrder, Config, MODE_0, Spi};
use utils::MemCell;

const SPEED: f32 = 0.3;

#[unsafe(link_section = ".sram4.blinky")]
static BUFFER: MemCell<[u16; 25]> = MemCell::uninit();

#[embassy_executor::task]
pub async fn task(p: BlinkySrc) -> ! {
    let mut t = utils::init_ticker!(1);

    // Safety: BUFFER is only used in this task.
    let buf: _ = unsafe { &mut *BUFFER.init([0; _]) };

    let mut config = Config::default();
    config.mode = MODE_0;
    config.bit_order = BitOrder::LsbFirst;
    config.frequency = mhz(12);
    config.miso_pull = Pull::None;
    config.gpio_speed = Speed::Medium;

    let mut led = Spi::new_txonly_nosck(p.spi_p, p.led_pin, p.dma, config);

    let mut hue = 0.;

    loop {
        let (r, g, b) = color_wheel(hue as _);
        hue = (hue + SPEED) % 1536.;

        ws2812_calc(buf, r, g, b);
        let _ = led.write(buf).await;

        t.next().await
    }
}

/// # Calculate WS2812 Data Buffer
/// Prepares the data buffer for WS2812 LED based on RGB values.
fn ws2812_calc<'t>(buf: &mut [u16; 25], r: u8, g: u8, b: u8) {
    const N0: u16 = 0b1111_0000_0000_0000; // bit 0
    const N1: u16 = 0b1111_1111_1100_0000; // bit 1

    let mut temp = [0; 25];
    for i in 0..8 {
        temp[i + 0] = if (g << i) & 0x80 != 0 { N1 } else { N0 };
        temp[i + 8] = if (r << i) & 0x80 != 0 { N1 } else { N0 };
        temp[i + 16] = if (b << i) & 0x80 != 0 { N1 } else { N0 };
    }

    buf.copy_from_slice(&temp[..])
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
