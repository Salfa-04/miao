//!
//! # System Module
//!

#![allow(dead_code)]
#![allow(unused_imports)]

///
/// # Device Enumeration
///
#[repr(usize)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum Device {
    Placeholder = 0x0000,
}

///
/// # Watch List of Monitored Devices
///
pub const WATCH_LIST: &[Device] = &[
    // Device::Placeholder,
];

/// Settings for Heartbeat Monitoring
impl Device {
    /// Health Check Interval in ms
    pub(self) const HEALTH_MS: u8 = 100;
    /// Device Expiration Time in ms
    pub(self) const EXPIRE_MS: u16 = 500;
}

mod devices;
mod heartbeat;
mod interrupts;
mod resources;
mod status;

pub use interrupts::Irqs;
pub use resources::*;
pub use status::SysMode;

/// # Private Imports
mod private {
    pub use assign_resources::assign_resources;
    pub use utils::{atomic, bitenum, prelude::*};

    pub use super::heartbeat::HeartBeat;
    pub use super::{Device, WATCH_LIST};

    pub use hal::bind_interrupts;
    pub use hal::{Peri, peripherals};
    pub use time::Ticker;

    pub use atomic::Ordering::Relaxed as Order;
    pub use atomic::{AtomicBool, AtomicI8};
}
