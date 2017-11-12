use std::{mem, str};
use std::iter::FromIterator;

use error::Error;

#[inline]
pub fn default<T: Default>(value: &mut Option<T>) -> T {
    mem::replace(value, None).unwrap_or_default()
}

#[inline]
pub fn optional<T>(value: &mut Option<T>) -> Option<T> {
    mem::replace(value, None)
}

#[inline]
pub fn required<T>(name: &str, value: &mut Option<T>) -> Result<T, Error> {
    mem::replace(value, None).ok_or_else(|| Error::missing_field(name))
}

#[inline]
pub fn iter<F, T, U, C>(data: &mut Vec<T>, f: F) -> Result<C, Error>
where
    C: FromIterator<U>,
    F: Fn(T) -> Result<U, Error>,
{
    data.drain(..).map(f).collect()
}

#[inline]
pub fn parse_key<K, Q, V>(entry: (Q, V)) -> Result<(K, V), Error>
where
    K: str::FromStr<Err = Error>,
    Q: AsRef<str>,
{
    let (key, value) = entry;
    let key = K::from_str(key.as_ref())?;

    Ok((key, value))
}
