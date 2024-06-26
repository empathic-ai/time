//! Re-implementation of [`std::time::SystemTime`].

//use std::error::Error;
//use crate::error::Error;
//use std::fmt::{self, Display, Formatter};
use core::fmt::{self, Display, Formatter};
//use std::ops::{Add, AddAssign, Sub, SubAssign};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use std::alloc::System;
use std::time::Duration;
//use crate::{Duration};

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

/// See [`std::time::SystemTime`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemTime(i64);

impl SystemTime {
	/// See [`std::time::SystemTime::UNIX_EPOCH`].
	pub const UNIX_EPOCH: Self = Self(0);

	pub fn new(val: i64) -> Self {
		Self(val)
	}

	/// See [`std::time::SystemTime::now()`].
	#[must_use]
	pub fn now() -> Self {
		#[cfg(all(
			target_family = "wasm",
			not(any(target_os = "emscripten", target_os = "wasi")),
			feature = "wasm-bindgen"
		))]
		#[allow(clippy::as_conversions)]
		return Self(js_sys::Date::now() as i64);

		#[cfg(feature = "std")]
		return Self(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64);
	}

	/// See [`std::time::SystemTime::duration_since()`].
	#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
	pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
		if self.0 < earlier.0 {
			let duration = (earlier.0 - self.0).try_into().unwrap();

			Err(SystemTimeError(Duration::from_millis(duration)))
		} else {
			let duration = (self.0 - earlier.0).try_into().unwrap();
			Ok(Duration::from_millis(duration))
		}
	}

	/// See [`std::time::SystemTime::elapsed()`].
	#[allow(clippy::missing_errors_doc)]
	pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
		Self::now().duration_since(*self)
	}

	/// See [`std::time::SystemTime::checked_add()`].
	pub fn checked_add_std(&self, duration: Duration) -> Option<Self> {
		let duration = duration.as_millis().try_into().ok()?;
		self.0.checked_add(duration).map(SystemTime)
	}

    	/// See [`std::time::SystemTime::checked_add()`].
	pub fn checked_add(&self, duration: crate::Duration) -> Option<Self> {
		let duration = duration.as_millis().try_into().ok()?;
		self.0.checked_add(duration).map(SystemTime)
	}

	/// See [`std::time::SystemTime::checked_sub()`].
	pub fn checked_sub_std(&self, duration: Duration) -> Option<Self> {
		let duration = duration.as_millis().try_into().ok()?;
		self.0.checked_sub(duration).map(SystemTime)
	}

    pub fn checked_sub(&self, duration: crate::Duration) -> Option<Self> {
		let duration = duration.as_millis().try_into().ok()?;
		self.0.checked_sub(duration).map(SystemTime)
	}
}

impl Add<crate::Duration> for SystemTime {
	type Output = Self;

	/// # Panics
	///
	/// This function may panic if the resulting point in time cannot be
	/// represented by the underlying data structure. See
	/// [`SystemTime::checked_add`] for a version without panic.
	fn add(self, dur: crate::Duration) -> Self {
		self.checked_add(dur)
			.expect("overflow when adding duration to instant")
	}
}

impl Add<Duration> for SystemTime {
	type Output = Self;

	/// # Panics
	///
	/// This function may panic if the resulting point in time cannot be
	/// represented by the underlying data structure. See
	/// [`SystemTime::checked_add_std`] for a version without panic.
	fn add(self, dur: Duration) -> Self {
		self.checked_add_std(dur)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for SystemTime {
	fn add_assign(&mut self, other: Duration) {
		*self = *self + other;
	}
}

impl Sub<Duration> for SystemTime {
	type Output = Self;

	fn sub(self, dur: Duration) -> Self {
		self.checked_sub_std(dur)
			.expect("overflow when subtracting duration from instant")
	}
}

impl SubAssign<Duration> for SystemTime {
	fn sub_assign(&mut self, other: Duration) {
		*self = *self - other;
	}
}

impl From<SystemTime> for std::time::SystemTime {
    fn from(value: SystemTime) -> Self {
        Self::UNIX_EPOCH + value.duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }
}

impl From<std::time::SystemTime> for SystemTime {
    fn from(value: std::time::SystemTime) -> Self {
        Self::UNIX_EPOCH + value.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap()
    }
}

/// See [`std::time::SystemTimeError`].
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
	/// See [`std::time::SystemTimeError::duration()`].
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn duration(&self) -> Duration {
		self.0
	}
}

impl Display for SystemTimeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "second time provided was later than self")
	}
}

//impl Error for SystemTimeError {}
