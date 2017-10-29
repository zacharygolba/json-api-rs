use std::mem;

use error::Error;
use value::Map;

pub fn default<T: Default>(value: &mut Option<T>) -> T {
    mem::replace(value, None).unwrap_or_default()
}

pub fn map<F, T, U>(data: &mut Vec<(String, T)>, f: F) -> Result<Map<U>, Error>
where
    F: Fn(T) -> Result<U, Error>,
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

pub fn vec<F, T, U>(data: &mut Vec<T>, f: F) -> Result<Vec<U>, Error>
where
    F: Fn(T) -> Result<U, Error>,
{
    data.drain(..).map(f).collect()
}
