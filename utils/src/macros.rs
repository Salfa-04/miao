//!
//! # Macros
//!

///
/// # init_ticker
///
/// Initialize a Ticker with a given period.
///
/// `init_ticker!()` initializes a Ticker with a given period.
///
/// ## Example
/// ```
/// let mut t = init_ticker!(500); // 500ms
///
/// (do something here..)
///
/// t.reset(); // Reset the ticker
///
/// loop {
///   // Do something
///    t.next().await;
/// }
///
/// ```
///
#[macro_export]
macro_rules! init_ticker {
    () => {{
        use ::defmt::trace;
        use $crate::prelude::time::{Duration, Ticker};

        trace!("{}: Ticker Initialized with 1 ms", file!());
        Ticker::every(Duration::from_millis(1))
    }};

    ($val:expr) => {{
        use ::defmt::trace;
        use $crate::prelude::time::{Duration, Ticker};

        trace!("{}: Ticker Initialized with {} ms", file!(), $val);
        Ticker::every(Duration::from_millis($val))
    }};

    ($val:expr, ms) => {{
        use ::defmt::trace;
        use $crate::prelude::time::{Duration, Ticker};

        trace!("{}: Ticker Initialized with {} ms", file!(), $val);
        Ticker::every(Duration::from_millis($val))
    }};

    ($val:expr, s) => {{
        use ::defmt::trace;
        use $crate::prelude::time::{Duration, Ticker};

        trace!("{}: Ticker Initialized with {} s", file!(), $val);
        Ticker::every(Duration::from_secs($val))
    }};
}
