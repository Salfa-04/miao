//!
//! # System Heartbeat
//!

use super::private::*;

///
/// # Heartbeat Structure
///
pub struct HeartBeat {
    online: AtomicBool,
    ttl: AtomicI8,
}

impl HeartBeat {
    ///
    ///  # Feed Heartbeat
    ///
    /// Set the device as online and reset its TTL (Time-To-Live) counter.
    ///
    pub fn feed(&self, ttl: i8) {
        self.online.store(true, Order);
        self.ttl.store(ttl, Order);
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Set the device as offline and reset its TTL (Time-To-Live) counter to zero.
    ///
    pub fn kill(&self) {
        self.online.store(false, Order);
        self.ttl.store(0, Order);
    }

    ///
    /// # Check Online Status
    ///
    /// Returns `true` if the device is online, `false` otherwise.
    ///
    pub fn check(&self) -> bool {
        self.online.load(Order)
    }

    ///
    /// # Get TTL
    ///
    /// Returns the current TTL value.
    ///
    pub fn ttl(&self) -> i8 {
        self.ttl.load(Order)
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter.
    ///
    /// If the counter reaches zero, mark the device as offline.
    ///
    pub fn tick(&self) -> bool {
        let prev = self.ttl.fetch_sub(1, Order);
        if prev < 1 {
            self.ttl.store(0, Order);
            self.online.store(false, Order);
            return false; // Offline
        }

        true // Still Online
    }
}
