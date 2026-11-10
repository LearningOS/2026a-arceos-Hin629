//! A hash map backed by hashbrown and seeded by the ArceOS random source.

use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::ops::Index;

use super::random::RandomState;
pub use hashbrown::hash_map::{
    Drain, Entry, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, OccupiedEntry, VacantEntry,
    Values, ValuesMut,
};
use hashbrown::Equivalent;

/// A hash map using system-generated seeds by default.
#[derive(Clone)]
pub struct HashMap<K, V, S = RandomState> {
    inner: hashbrown::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, RandomState> {
    /// Creates an empty map without allocating buckets.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty map with room for at least `capacity` entries.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, RandomState::new())
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty map using the supplied hash builder.
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            inner: hashbrown::HashMap::with_hasher(hash_builder),
        }
    }

    /// Creates a map with the requested capacity and hash builder.
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            inner: hashbrown::HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of entries that fit without reallocating.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns the map's hash builder.
    pub fn hasher(&self) -> &S {
        self.inner.hasher()
    }

    /// Removes all entries, retaining capacity.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Iterates over entries in arbitrary order.
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.inner.iter()
    }

    /// Iterates over keys and mutable values in arbitrary order.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.inner.iter_mut()
    }

    /// Iterates over keys.
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.inner.keys()
    }

    /// Iterates over values.
    pub fn values(&self) -> Values<'_, K, V> {
        self.inner.values()
    }

    /// Iterates over mutable values.
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.inner.values_mut()
    }

    /// Consumes the map and iterates over keys.
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.inner.into_keys()
    }

    /// Consumes the map and iterates over values.
    pub fn into_values(self) -> IntoValues<K, V> {
        self.inner.into_values()
    }

    /// Drains all entries, retaining capacity.
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        self.inner.drain()
    }

    /// Keeps only entries accepted by `predicate`.
    pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, predicate: F) {
        self.inner.retain(predicate);
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> HashMap<K, V, S> {
    /// Inserts a value, returning the old value if present.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Returns the value matching a borrowed query key.
    pub fn get<Q: ?Sized + Hash + Equivalent<K>>(&self, key: &Q) -> Option<&V> {
        self.inner.get(key)
    }

    /// Returns the stored key and value matching the query.
    pub fn get_key_value<Q: ?Sized + Hash + Equivalent<K>>(&self, key: &Q) -> Option<(&K, &V)> {
        self.inner.get_key_value(key)
    }

    /// Returns a mutable reference to the matching value.
    pub fn get_mut<Q: ?Sized + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    /// Returns whether a matching key exists.
    pub fn contains_key<Q: ?Sized + Hash + Equivalent<K>>(&self, key: &Q) -> bool {
        self.inner.contains_key(key)
    }

    /// Removes an entry and returns its value.
    pub fn remove<Q: ?Sized + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<V> {
        self.inner.remove(key)
    }

    /// Removes an entry and returns its stored key and value.
    pub fn remove_entry<Q: ?Sized + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<(K, V)> {
        self.inner.remove_entry(key)
    }

    /// Provides access to an occupied or vacant entry.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S> {
        self.inner.entry(key)
    }

    /// Reserves space for at least `additional` more entries.
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Attempts to reserve space, returning allocation or capacity errors.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), hashbrown::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Shrinks the allocation as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks without going below `min_capacity` or the length.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

impl<K, V, S: Default> Default for HashMap<K, V, S> {
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for HashMap<K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<K: Eq + Hash, V: PartialEq, S: BuildHasher> PartialEq for HashMap<K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K: Eq + Hash, V: Eq, S: BuildHasher> Eq for HashMap<K, V, S> {}

impl<K, V, S> IntoIterator for HashMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Extend<(K, V)> for HashMap<K, V, S> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl<'a, K: 'a + Eq + Hash + Copy, V: 'a + Copy, S: BuildHasher> Extend<(&'a K, &'a V)>
    for HashMap<K, V, S>
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> FromIterator<(K, V)> for HashMap<K, V, S> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<K: Eq + Hash, V, const N: usize> From<[(K, V); N]> for HashMap<K, V> {
    fn from(entries: [(K, V); N]) -> Self {
        entries.into_iter().collect()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, Q: ?Sized + Hash + Equivalent<K>> Index<&Q>
    for HashMap<K, V, S>
{
    type Output = V;
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}
