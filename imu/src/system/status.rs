//!
//! # System Status
//!

use super::private::*;

static STATUS: AtomicI8 = AtomicI8::new(SysMode::Boot.into_bits());

///
/// # System Mode Enumeration
///
/// ## Get Current Mode
/// ```rust
/// let mode: SysMode = SysMode::get();
/// ```
///
/// ## Set Current Mode
/// ```rust
/// SysMode::Normal.set();
/// SysMode::set(SysMode::Normal);
/// ```
///
#[repr(i8)]
#[bitenum]
#[non_exhaustive]
#[derive(PartialEq, defmt::Format, Debug)]
pub enum SysMode {
    #[fallback]
    Error = -1,
    Boot = 0,
    Normal = 1,
}

impl SysMode {
    ///
    /// # Get System Mode
    ///
    /// Retrieve the current system mode.
    ///
    #[inline]
    pub fn get() -> SysMode {
        SysMode::from_bits(STATUS.load(Order))
    }

    ///
    /// # Set System Mode
    ///
    /// Set the current system mode to the specified value.
    ///
    #[inline]
    pub fn set(self) {
        STATUS.store(self.into_bits(), Order);
    }

    ///
    /// # Wait for System Mode
    ///
    /// Wait until the system mode matches the specified mode.
    ///
    pub fn wait(&self, t: &mut Ticker) -> impl Future<Output = ()> {
        async {
            while Self::get() != *self {
                t.next().await
            }
        }
    }
}
