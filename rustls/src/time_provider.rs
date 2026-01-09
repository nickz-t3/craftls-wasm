//! The library's source of time.

use core::fmt::Debug;

use pki_types::UnixTime;

/// An object that provides the current time.
pub trait TimeProvider: Debug + Send + Sync {
    /// Returns the current wall time.
    fn current_time(&self) -> Option<UnixTime>;
}

/// Default `TimeProvider` implementation that uses `std`
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct DefaultTimeProvider;

#[cfg(feature = "std")]
impl TimeProvider for DefaultTimeProvider {
    fn current_time(&self) -> Option<UnixTime> {
        Some(UnixTime::now())
    }
}
