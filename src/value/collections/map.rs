//! A hash map implementation with consistent ordering.
//!
//! The types in this module are commonly used as the underlying data structure of
//! arbitrary objects found in JSON API data.

use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::RangeFull;

use ordermap::{self, OrderMap};
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use value::{Key, Value};
use value::collections::Equivalent;

/// A hash map implementation with consistent ordering.
#[derive(Clone, Eq, PartialEq)]
pub struct Map<K = Key, V = Value>
where
    K: Eq + Hash,
{
    inner: OrderMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash,
{
    /// Creates an empty `Map`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # fn main() {
    /// use json_api::value::{Key, Map, Value};
    /// let mut map = Map::<Key, Value>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new empty `Map`, with specified capacity.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Map;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let mut map = Map::with_capacity(2);
    ///
    /// map.insert("x", 1);
    /// map.insert("y", 2);
    ///
    /// // The next insert will likely require reallocation...
    /// map.insert("z", 3);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = OrderMap::with_capacity(capacity);
        Map { inner }
    }

    /// Returns the number of key-value pairs the map can hold without reallocating.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::{Key, Map, Value};
    /// #
    /// # fn main() {
    /// let map = Map::<Key, Value>::with_capacity(2);
    /// assert!(map.capacity() >= 2);
    /// # }
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("x", 1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// # }
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash,
    {
        self.inner.contains_key(key)
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the allocated
    /// memory for reuse.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("x", 1);
    /// map.insert("y", 2);
    ///
    /// for (key, value) in map.drain(..) {
    ///     assert!(key == "x" || key == "y");
    ///     assert!(value == 1 || value == 2);
    /// }
    ///
    /// assert!(map.is_empty());
    /// # }
    /// ```
    pub fn drain(&mut self, range: RangeFull) -> Drain<K, V> {
        let iter = self.inner.drain(range);
        Drain { iter }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("x", 1);
    ///
    /// assert_eq!(map.get("x"), Some(&1));
    /// assert_eq!(map.get("y"), None);
    /// # }
    /// ```
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        Q: Equivalent<K> + Hash,
    {
        self.inner.get(key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If a value already existed for key, that old value is returned in `Some`;
    /// otherwise, `None` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// assert_eq!(map.insert("x", 1), None);
    /// assert_eq!(map.insert("x", 2), Some(1));
    /// # }
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Return an iterator visiting all the key-value pairs of the map in the order
    /// in which they were inserted.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {} value: {}", key, value);
    /// }
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        let iter = self.inner.iter();
        Iter { iter }
    }

    /// Return an iterator visiting all the key-value pairs of the map in the order
    /// in which they were inserted, with mutable references to the values.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value += 1;
    /// }
    ///
    /// for (key, value) in &map {
    ///     println!("key: {} value: {}", key, value);
    /// }
    /// # }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        let iter = self.inner.iter_mut();
        IterMut { iter }
    }

    /// Return an iterator visiting all keys in the order in which they were inserted.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    /// # }
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        let iter = self.inner.keys();
        Keys { iter }
    }

    /// Return the number of key-value pairs in the map.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    /// assert_eq!(map.len(), 0);
    ///
    /// map.insert("x", 1);
    /// assert_eq!(map.len(), 1);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    /// assert!(map.is_empty());
    ///
    /// map.insert("x", 1);
    /// assert!(!map.is_empty());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes a key from the map, returning the value at the key if the key was
    /// previously in the map.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("x", 1);
    ///
    /// assert_eq!(map.remove("x"), Some(1));
    /// assert_eq!(map.remove("x"), None);
    /// # }
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        Q: Equivalent<K> + Hash,
    {
        self.inner.remove(key)
    }


    /// Reserves capacity for at least additional more elements to be inserted
    /// in the `Map`. The collection may reserve more space to avoid frequent
    /// reallocations.
    ///
    /// # Note
    ///
    /// This method has yet to be fully implemented in the [`ordermap`] crate.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::{Key, Map, Value};
    /// #
    /// # fn main() {
    /// let mut map = Map::<Key, Value>::new();
    /// map.reserve(10);
    /// # }
    /// ```
    ///
    /// [`ordermap`]: https://docs.rs/ordermap
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Return an iterator visiting all values in the order in which they were inserted.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for value in map.values() {
    ///     println!("{}", value);
    /// }
    /// # }
    /// ```
    pub fn values(&self) -> Values<K, V> {
        let iter = self.inner.values();
        Values { iter }
    }

    /// Return an iterator visiting all values mutably in the order in which they were
    /// inserted.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Map;
    /// #
    /// # fn main() {
    /// let mut map = Map::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for value in map.values_mut() {
    ///     *value += 1;
    /// }
    ///
    /// for value in map.values() {
    ///     println!("{}", value);
    /// }
    /// # }
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        let iter = self.inner.values_mut();
        ValuesMut { iter }
    }
}

impl<K, V> Debug for Map<K, V>
where
    K: Debug + Eq + Hash,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        let inner = Default::default();
        Map { inner }
    }
}

impl<K, V> Extend<(K, V)> for Map<K, V>
where
    K: Eq + Hash,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        self.inner.extend(iter);
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let inner = OrderMap::from_iter(iter);
        Map { inner }
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.into_iter();
        IntoIter { iter }
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'de, K, V> Deserialize<'de> for Map<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        OrderMap::deserialize(deserializer).map(|inner| Map { inner })
    }
}

impl<K, V> Serialize for Map<K, V>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

/// A draining iterator over the entries of a `Map`.
pub struct Drain<'a, K: 'a, V: 'a> {
    iter: ordermap::Drain<'a, K, V>,
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// An iterator over the entries of a `Map`.
pub struct Iter<'a, K: 'a, V: 'a> {
    iter: ordermap::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An mutable iterator over the entries of a `Map`.
pub struct IterMut<'a, K: 'a, V: 'a> {
    iter: ordermap::IterMut<'a, K, V>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An owning iterator over the entries of a `Map`.
pub struct IntoIter<K, V> {
    iter: ordermap::IntoIter<K, V>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An iterator over the keys of a `Map`.
pub struct Keys<'a, K: 'a, V: 'a> {
    iter: ordermap::Keys<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An iterator over the values of a `Map`.
pub struct Values<'a, K: 'a, V: 'a> {
    iter: ordermap::Values<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// A mutable iterator over the values of a `Map`.
pub struct ValuesMut<'a, K: 'a, V: 'a> {
    iter: ordermap::ValuesMut<'a, K, V>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}
