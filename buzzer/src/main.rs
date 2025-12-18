#![no_std]
#![no_main]

use utils::prelude::*;

mod controller;
mod system;

mod tasks {
    pub mod blinky;
    pub mod buzzer;
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
        scb.enable_icache();
        let i = SCB::icache_enabled();
        let d = SCB::dcache_enabled();
        defmt::trace!("ICache: {}, DCache: {}", i, d);
    }

    s.must_spawn(tasks::health::task());

    s.must_spawn(tasks::blinky::task(r.blinky));

    s.must_spawn(tasks::buzzer::task(r.buzzer));

    s.must_spawn(controller::main());
}
