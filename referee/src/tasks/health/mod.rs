//!
//! # Health Task
//!

use crate::{system::*, time::Instant};
use utils::init_ticker;

#[embassy_executor::task]
pub async fn task() -> ! {
    let mut t = init_ticker!(Device::interval(), ms);

    let mut last = Instant::now();

    loop {
        for device in WATCH_LIST {
            if !device.tick() {
                SysMode::Error.set();
            }
        }

        if last.elapsed().as_secs() >= 1 {
            last = Instant::now();
            for ele in WATCH_LIST {
                if !ele.check() {
                    defmt::warn!("{:?}", ele.display());
                }
            }
        }

        t.next().await
    }
}
