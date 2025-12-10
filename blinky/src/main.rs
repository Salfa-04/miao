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

    s.must_spawn(tasks::health::task());

    s.must_spawn(tasks::blinky::task(r.blinky));

    s.must_spawn(controller::main());
}
