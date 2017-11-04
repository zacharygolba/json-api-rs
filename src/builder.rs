use std::hash::Hash;
use std::iter::FromIterator;
use std::mem;
use std::str::FromStr;

use error::Error;
use value::Map;

pub fn default<T: Default>(value: &mut Option<T>) -> T {
    mem::replace(value, None).unwrap_or_default()
}

pub fn map<F, K, T, U>(data: &mut Vec<(String, T)>, f: F) -> Result<Map<K, U>, Error>
where
    F: Fn(T) -> Result<U, Error>,
    K: Eq + FromStr<Err = Error> + Hash,
{
    data.drain(..)
        .map(|(key, value)| Ok((key.parse()?, f(value)?)))
        .collect()
}

pub fn optional<T>(value: &mut Option<T>) -> Option<T> {
    mem::replace(value, None)
}

pub fn required<T>(name: &str, value: &mut Option<T>) -> Result<T, Error> {
    mem::replace(value, None).ok_or_else(|| Error::missing_field(name))
}

pub fn iter<F, T, U, C>(data: &mut Vec<T>, f: F) -> Result<C, Error>
where
    C: FromIterator<U>,
    F: Fn(T) -> Result<U, Error>,
{
    data.drain(..).map(f).collect()
}
