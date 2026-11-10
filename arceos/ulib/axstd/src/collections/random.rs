use core::{
    fmt,
    hash::{BuildHasher, Hasher},
};

#[allow(deprecated)]
use core::hash::SipHasher;

use arceos_api::modules::axhal::misc::random;

/// `RandomState` is the default state for [`HashMap`] types.
///
/// A particular instance `RandomState` will create the same instances of
/// [`Hasher`], but the hashers created by two different `RandomState`
/// instances are unlikely to produce the same result for the same values.
///
/// [`HashMap`]: crate::collections::HashMap
///
/// # Examples
///
/// ```
/// use axstd::collections::{HashMap, RandomState};
///
/// let s = RandomState::new();
/// let mut map = HashMap::with_hasher(s);
/// map.insert(1, 2);
/// ```
#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    /// Constructs a new `RandomState` that is initialized with random keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use axstd::collections::RandomState;
    ///
    /// let s = RandomState::new();
    /// ```
    #[inline]
    pub fn new() -> RandomState {
        let num = random();
        Self {
            k0: num as u64,
            k1: (num >> 64) as u64,
        }
    }
}
#[allow(deprecated)]
impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(self.k0, self.k1))
    }
}

/// The default [`Hasher`] used by [`RandomState`].
///
/// The internal algorithm is not specified, and so it and its hashes should
/// not be relied upon over releases.
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher);

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[inline]
    #[allow(deprecated)]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    // The underlying `SipHasher` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl Default for RandomState {
    /// Constructs a new `RandomState`.
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}

impl fmt::Debug for RandomState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomState").finish_non_exhaustive()
    }
}
