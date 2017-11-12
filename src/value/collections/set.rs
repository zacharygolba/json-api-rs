//! A hash set implemented as a `Map` where the value is `()`.

use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::RangeFull;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use value::Key;
use value::collections::Equivalent;
use value::collections::map::{self, Keys, Map};

/// A hash set implemented as a `Map` where the value is `()`.
#[derive(Clone, Eq, PartialEq)]
pub struct Set<T: Eq + Hash = Key> {
    inner: Map<T, ()>,
}

impl<T: Eq + Hash> Set<T> {
    /// Creates an empty `Set`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # fn main() {
    /// use json_api::value::{Key, Set};
    /// let mut set = Set::<Key>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new empty `Set`, with specified capacity.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Set;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let mut set = Set::with_capacity(2);
    ///
    /// set.insert("x");
    /// set.insert("y");
    ///
    /// // The next insert will likely require reallocation...
    /// set.insert("z");
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = Map::with_capacity(capacity);
        Set { inner }
    }

    /// Returns the number of elements the set can hold without reallocating.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::{Key, Set};
    /// #
    /// # fn main() {
    /// let set = Set::<Key>::with_capacity(2);
    /// assert!(set.capacity() >= 2);
    /// # }
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the set, removing all elements. Keeps the allocated memory for
    /// reuse.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// set.insert("x");
    /// set.clear();
    /// assert!(set.is_empty());
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the set contains the specified value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// set.insert(1);
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&2));
    /// # }
    /// ```
    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        Q: Equivalent<T> + Hash,
    {
        self.inner.contains_key(key)
    }

    /// Clears the set, returning all elements in an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// set.insert(1);
    /// set.insert(2);
    ///
    /// for item in set.drain(..) {
    ///     assert!(item == 1 || item == 2);
    /// }
    ///
    /// assert!(set.is_empty());
    /// # }
    /// ```
    pub fn drain(&mut self, range: RangeFull) -> Drain<T> {
        let iter = self.inner.drain(range);
        Drain { iter }
    }

    /// Adds a value to the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// assert_eq!(set.insert(1), true);
    /// assert_eq!(set.insert(1), false);
    /// assert_eq!(set.len(), 1);
    /// # }
    /// ```
    pub fn insert(&mut self, key: T) -> bool {
        self.inner.insert(key, ()).is_none()
    }

    /// Returns true if the set does not contain any elements.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    /// assert!(set.is_empty());
    ///
    /// set.insert("x");
    /// assert!(!set.is_empty());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return an iterator visiting all the elements of the set in the order in
    /// which they were inserted.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// set.insert("a");
    /// set.insert("b");
    /// set.insert("c");
    ///
    /// let mut iter = set.iter();
    ///
    /// assert_eq!(iter.next(), Some(&"a"));
    /// assert_eq!(iter.next(), Some(&"b"));
    /// assert_eq!(iter.next(), Some(&"c"));
    /// assert_eq!(iter.next(), None);
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        let iter = self.inner.keys();
        Iter { iter }
    }

    /// Return the number of elements in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    /// assert_eq!(set.len(), 0);
    ///
    /// set.insert("x");
    /// assert_eq!(set.len(), 1);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a value from the set. Returns `true` if the value was present
    /// in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::new();
    ///
    /// set.insert("x");
    ///
    /// assert!(set.remove("x"));
    /// assert!(!set.remove("x"));
    /// assert_eq!(set.len(), 0);
    /// # }
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> bool
    where
        Q: Equivalent<T> + Hash,
    {
        self.inner.remove(key).is_some()
    }

    /// Reserves capacity for at least additional more elements to be inserted
    /// in the `Set`. The collection may reserve more space to avoid frequent
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
    /// # use json_api::value::Set;
    /// #
    /// # fn main() {
    /// let mut set = Set::<String>::new();
    /// set.reserve(10);
    /// # }
    /// ```
    ///
    /// [`ordermap`]: https://docs.rs/ordermap
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }
}

impl<T: Debug + Eq + Hash> Debug for Set<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

impl<T: Eq + Hash> Default for Set<T> {
    fn default() -> Self {
        let inner = Default::default();
        Set { inner }
    }
}

impl<T: Display + Eq + Hash> Display for Set<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let sep = ',';

        for (idx, key) in self.iter().enumerate() {
            if idx > 0 {
                Display::fmt(&sep, f)?;
            }

            Display::fmt(key, f)?;
        }

        Ok(())
    }
}

impl<T: Eq + FromStr + Hash> Extend<T> for Set<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter().map(|key| (key, ()));
        self.inner.extend(iter);
    }
}

impl<T: Eq + FromStr + Hash> FromIterator<T> for Set<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let inner = iter.into_iter().map(|key| (key, ())).collect();
        Set { inner }
    }
}

impl<T: Eq + Hash> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.into_iter();
        IntoIter { iter }
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a Set<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'de, T, E> Deserialize<'de> for Set<T>
where
    E: Display + 'de,
    T: Deserialize<'de> + Eq + FromStr<Err = E> + Hash + 'de,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        struct SetVisitor<'de, T, D>
        where
            D: Display + 'de,
            T: Deserialize<'de> + Eq + FromStr<Err = D> + Hash + 'de,
        {
            err: PhantomData<&'de D>,
            key: PhantomData<&'de T>,
        }

        impl<'de, T, D> SetVisitor<'de, T, D>
        where
            D: Display,
            T: Deserialize<'de> + Eq + FromStr<Err = D> + Hash,
        {
            fn new() -> Self {
                SetVisitor {
                    err: PhantomData,
                    key: PhantomData,
                }
            }
        }

        impl<'de, T, D> Visitor<'de> for SetVisitor<'de, T, D>
        where
            D: Display,
            T: Deserialize<'de> + Eq + FromStr<Err = D> + Hash,
        {
            type Value = Set<T>;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("a sequence of json api member names")
            }

            fn visit_str<E: Error>(self, data: &str) -> Result<Self::Value, E> {
                data.split(',')
                    .map(|item| item.trim().parse().map_err(Error::custom))
                    .collect()
            }
        }

        deserializer.deserialize_seq(SetVisitor::new())
    }
}

impl<T: Display + Eq + Hash> Serialize for Set<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// A draining iterator over the items of a `Set`.
pub struct Drain<'a, T: 'a> {
    iter: map::Drain<'a, T, ()>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _)| key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// An iterator over the items of a `Set`.
pub struct Iter<'a, T: 'a> {
    iter: Keys<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

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

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An owning iterator over the items of a `Set`.
pub struct IntoIter<T> {
    iter: map::IntoIter<T, ()>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _)| key)
    }

    fn count(self) -> usize {
        self.iter.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|(key, _)| key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(key, _)| key)
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}
