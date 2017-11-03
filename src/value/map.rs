use std::iter::FromIterator;
use std::ops::RangeFull;
use std::fmt::{self, Debug, Formatter};

use ordermap::{self, OrderMap};
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::Key;

#[derive(Clone, PartialEq)]
pub struct Map<T> {
    inner: OrderMap<Key, T>,
}

impl<T> Map<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let inner = OrderMap::with_capacity(capacity);
        Map { inner }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn drain(&mut self, range: RangeFull) -> Drain<T> {
        let iter = self.inner.drain(range);
        Drain { iter }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get(key)
    }

    pub fn insert(&mut self, key: Key, value: T) -> Option<T> {
        self.inner.insert(key, value)
    }

    pub fn iter(&self) -> Iter<T> {
        let iter = self.inner.iter();
        Iter { iter }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let iter = self.inner.iter_mut();
        IterMut { iter }
    }

    pub fn keys(&self) -> Keys<T> {
        let iter = self.inner.keys();
        Keys { iter }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.inner.remove(key)
    }

    pub fn values(&self) -> Values<T> {
        let iter = self.inner.values();
        Values { iter }
    }

    pub fn values_mut(&mut self) -> ValuesMut<T> {
        let iter = self.inner.values_mut();
        ValuesMut { iter }
    }
}

impl<T: Debug> Debug for Map<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        let inner = Default::default();
        Map { inner }
    }
}

impl<T> Extend<(Key, T)> for Map<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Key, T)>,
    {
        self.inner.extend(iter);
    }
}

impl<T> FromIterator<(Key, T)> for Map<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Key, T)>,
    {
        let inner = OrderMap::from_iter(iter);
        Map { inner }
    }
}

impl<T> IntoIterator for Map<T> {
    type Item = (Key, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.inner.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type Item = (&'a Key, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Map<T> {
    type Item = (&'a Key, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'de, T> Deserialize<'de> for Map<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        OrderMap::deserialize(deserializer).map(|inner| Map { inner })
    }
}

impl<T: Serialize> Serialize for Map<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

pub struct Drain<'a, T: 'a> {
    iter: ordermap::Drain<'a, Key, T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = (Key, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct Iter<'a, T: 'a> {
    iter: ordermap::Iter<'a, Key, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a Key, &'a T);

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

pub struct IterMut<'a, T: 'a> {
    iter: ordermap::IterMut<'a, Key, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (&'a Key, &'a mut T);

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

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct IntoIter<T> {
    iter: ordermap::IntoIter<Key, T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (Key, T);

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

impl<'a, T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct Keys<'a, T: 'a> {
    iter: ordermap::Keys<'a, Key, T>,
}

impl<'a, T> Iterator for Keys<'a, T> {
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

impl<'a, T> DoubleEndedIterator for Keys<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Keys<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct Values<'a, T: 'a> {
    iter: ordermap::Values<'a, Key, T>,
}

impl<'a, T> Iterator for Values<'a, T> {
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

impl<'a, T> DoubleEndedIterator for Values<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Values<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct ValuesMut<'a, T: 'a> {
    iter: ordermap::ValuesMut<'a, Key, T>,
}

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

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

impl<'a, T> DoubleEndedIterator for ValuesMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator for ValuesMut<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}
