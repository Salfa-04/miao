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
    let (_c, p) = utils::sys_init();
    let r = {
        use system::*;
        split_resources!(p)
    };

    {
        let mut scb = _c.SCB;
        scb.enable_icache();

        use utils::peripheral::SCB;
        let a = SCB::dcache_enabled();
        let b = SCB::icache_enabled();
        defmt::info!("DCACHE: {}, ICACHE: {}", a, b,);
    }

    s.must_spawn(tasks::health::task());

    s.must_spawn(tasks::blinky::task(r.blinky));

    s.must_spawn(controller::main());
}
