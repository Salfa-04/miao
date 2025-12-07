#![no_std]
#![no_main]
#![allow(unused_imports)]

use ::defmt_rtt as _;
use ::panic_probe as _;

mod init;
mod macros;

pub use bitfield_struct::*;
pub use init::sys_init;
pub use prelude::ll::asm;
pub use prelude::ll::peripheral;
pub use prelude::time::Timer as T;
pub use static_cell::*;

/// # Atomic Types Module
pub mod atomic {
    pub use ::portable_atomic::*;
}

/// # Preludes for Easy Imports.
pub mod prelude {
    pub use ::cortex_m as ll; // Low Level
    pub use ::embassy_futures as ef; // Futures
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_sync as sync; // Sync
    pub use ::embassy_time as time; // Time
}

/// # Defmt Panic Handler
#[::defmt::panic_handler]
fn soft_panic() -> ! {
    ::panic_probe::hard_fault()
}
