use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::RangeFull;
use std::str::FromStr;

use ordermap::Equivalent;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor};

use value::map::{self, Keys, Map};

#[derive(Clone, Eq, PartialEq)]
pub struct Set<T: Eq + Hash> {
    inner: Map<T, ()>,
}

impl<T: Eq + Hash> Set<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let inner = Map::with_capacity(capacity);
        Set { inner }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<T> + Hash,
    {
        self.inner.contains_key(key)
    }

    pub fn drain(&mut self, range: RangeFull) -> Drain<T> {
        let iter = self.inner.drain(range);
        Drain { iter }
    }

    pub fn insert(&mut self, key: T) -> bool {
        self.inner.insert(key, ()).is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        let iter = self.inner.keys();
        Iter { iter }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        Q: Equivalent<T> + Hash,
    {
        self.inner.remove(key).is_some()
    }
}

impl<T: Debug + Eq + FromStr + Hash> Debug for Set<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
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
