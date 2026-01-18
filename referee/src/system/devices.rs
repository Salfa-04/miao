//!
//! # System Devices
//!

use super::private::*;

const LIST_SIZE: usize = WATCH_LIST.len();
type Pair = (&'static Device, &'static HeartBeat);

static PAIRS: Option<[Pair; LIST_SIZE]> = const {
    static STATE: [HeartBeat; LIST_SIZE] = unsafe { core::mem::zeroed() };
    match LIST_SIZE {
        0 => None,
        _ => {
            let mut pairs = [(&WATCH_LIST[0], &STATE[0]); LIST_SIZE];
            let mut i = 0;
            while i < LIST_SIZE {
                pairs[i] = (&WATCH_LIST[i], &STATE[i]);
                i += 1;
            }
            Some(pairs)
        }
    }
};

impl Device {
    ///
    /// # Get Heartbeat
    ///
    /// Get the heartbeat associated with this device.
    ///
    fn heartbeat(&self) -> Option<&'static HeartBeat> {
        PAIRS
            .as_ref()
            .and_then(|p: _| p.iter().find(|&&(addr, _)| addr == self))
            .map(|x: _| x.1)
    }

    ///
    /// # Maximum TTL
    ///
    /// Calculate the maximum Time-To-Live (TTL) value.
    ///
    const fn max_ttl() -> i8 {
        (Self::EXPIRE_MS / Self::HEALTH_MS as u16) as i8
    }

    ///
    /// # Get Health Check Interval
    ///
    /// Returns the health check interval in milliseconds.
    ///
    pub const fn interval() -> u64 {
        Self::HEALTH_MS as _
    }

    ///
    /// # Display Health
    ///
    /// Returns a Health formatter for this device.
    ///
    /// **impl [defmt::Format]**
    ///
    pub const fn display(&self) -> Display<'_> {
        Display { inner: self }
    }
}

impl Device {
    ///
    /// # Feed Heartbeat
    ///
    /// Feed the heartbeat for this device.
    ///
    pub fn feed(&self) {
        match self.heartbeat() {
            Some(x) => {
                x.feed(Self::max_ttl());
            }
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Kill the heartbeat for this device.
    ///
    pub fn kill(&self) {
        match self.heartbeat() {
            Some(x) => x.kill(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Check Heartbeat
    ///
    /// Check if the heartbeat for this device is alive.
    ///
    pub fn check(&self) -> bool {
        match self.heartbeat() {
            Some(x) => x.check(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Wait for Device to be Online
    ///
    /// Returns a future that resolves when the device is online.
    ///
    pub fn wait(&self, t: &mut Ticker) -> impl Future<Output = ()> {
        let heart = match self.heartbeat() {
            Some(x) => x,
            None => panic!("Invalid Address: {:?}", self),
        };

        async {
            while !heart.check() {
                t.next().await
            }
        }
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter.
    ///
    /// - `true` if the device is still online.
    /// - `false` if the device has gone offline.
    ///
    pub fn tick(&self) -> bool {
        match self.heartbeat() {
            Some(x) => x.tick(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }
}

pub struct Display<'t> {
    inner: &'t Device,
}

impl<'t> defmt::Format for Display<'t> {
    fn format(&self, fmt: defmt::Formatter) {
        let this = self.inner;
        let device: _ = this.heartbeat();

        if let Some(heart) = device {
            if heart.check() {
                defmt::write!(fmt, "{:?} (Online, TTL={})", this, heart.ttl());
            } else {
                defmt::write!(fmt, "{:?} (Offline)", this);
            }
        } else {
            defmt::write!(fmt, "{:?} (No Heartbeat)", this);
        }
    }
}
