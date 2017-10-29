use std::fmt::{self, Formatter};
use std::iter::FromIterator;
use std::ops::RangeFull;

use serde::ser::{Serialize, SerializeSeq, Serializer};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};

use super::Key;
use super::map::{self, Keys, Map};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Set {
    inner: Map<()>,
}

impl Set {
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

    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn drain(&mut self, range: RangeFull) -> Drain {
        let iter = self.inner.drain(range);
        Drain { iter }
    }

    pub fn insert(&mut self, key: Key) -> bool {
        self.inner.insert(key, ()).is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter {
        let iter = self.inner.keys();
        Iter { iter }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }
}

impl Extend<Key> for Set {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Key>,
    {
        let iter = iter.into_iter().map(|key| (key, ()));
        self.inner.extend(iter);
    }
}

impl FromIterator<Key> for Set {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Key>,
    {
        let inner = iter.into_iter().map(|key| (key, ())).collect();
        Set { inner }
    }
}

impl IntoIterator for Set {
    type Item = Key;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.into_iter();
        IntoIter { iter }
    }
}

impl<'a> IntoIterator for &'a Set {
    type Item = &'a Key;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'de> Deserialize<'de> for Set {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SetVisitor;

        impl<'de> Visitor<'de> for SetVisitor {
            type Value = Set;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("a sequence of json api member names")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut set = Set::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(value) = seq.next_element::<String>()? {
                    set.insert(value.parse().map_err(de::Error::custom)?);
                }

                Ok(set)
            }
        }

        deserializer.deserialize_seq(SetVisitor)
    }
}

impl Serialize for Set {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.len()))?;

        for key in self {
            state.serialize_element(key)?;
        }

        state.end()
    }
}

pub struct Drain<'a> {
    iter: map::Drain<'a, ()>,
}

impl<'a> Iterator for Drain<'a> {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _)| key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct Iter<'a> {
    iter: Keys<'a, ()>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Key;

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

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct IntoIter {
    iter: map::IntoIter<()>,
}

impl Iterator for IntoIter {
    type Item = Key;

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

impl<'a> DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(key, _)| key)
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.iter.len()
    }
}
