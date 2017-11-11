pub mod key;
pub mod map;
pub mod set;

use std::cmp::PartialEq;
use std::fmt::{self, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

use serde::de::{Deserialize, DeserializeOwned, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_json::{self, Value as JsonValue};

use error::Error;

pub use http::{StatusCode, Uri};
pub use serde_json::value::Number;

pub use self::key::Key;
pub use self::map::Map;
pub use self::set::Set;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Array(Vec<Value>),
    Bool(bool),
    Number(Number),
    Object(Map<Key, Value>),
    String(String),
}

impl Value {
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Map<Key, Value>> {
        match *self {
            Value::Object(ref inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Map<Key, Value>> {
        match *self {
            Value::Object(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(inner: bool) -> Self {
        Value::Bool(inner)
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Self {
        From::from(n as f64)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Number::from_f64(n).map_or(Value::Null, Value::Number)
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        From::from(n as i64)
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        From::from(n as i64)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        From::from(n as i64)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n.into())
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Self {
        From::from(n as u64)
    }
}

impl From<u16> for Value {
    fn from(n: u16) -> Self {
        From::from(n as u64)
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        From::from(n as u64)
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Number(n.into())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<Map<Key, Value>> for Value {
    fn from(data: Map<Key, Value>) -> Self {
        Value::Object(data)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(data: Option<T>) -> Self {
        data.map_or(Value::Null, T::into)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(data: Vec<T>) -> Self {
        Value::Array(data.into_iter().map(|i| i.into()).collect())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String(s.to_owned())
    }
}

impl<'a, T> From<&'a [T]> for Value
where
    T: Clone + Into<Value>,
{
    fn from(data: &'a [T]) -> Self {
        Value::Array(data.iter().cloned().map(|i| i.into()).collect())
    }
}

impl<T> FromIterator<T> for Value
where
    T: Into<Value>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Value::Array(iter.into_iter().map(|i| i.into()).collect())
    }
}

impl FromIterator<(Key, Value)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Key, Value)>,
    {
        Value::Object(Map::from_iter(iter))
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        from_json(src.parse()?)
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, rhs: &bool) -> bool {
        self.as_bool().map_or(false, |lhs| lhs == *rhs)
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, rhs: &f32) -> bool {
        self.as_f64().map_or(false, |lhs| lhs == (*rhs as f64))
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, rhs: &f64) -> bool {
        self.as_f64().map_or(false, |lhs| lhs == (*rhs as f64))
    }
}

impl PartialEq<i8> for Value {
    fn eq(&self, rhs: &i8) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == (*rhs as i64))
    }
}

impl PartialEq<i16> for Value {
    fn eq(&self, rhs: &i16) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == (*rhs as i64))
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, rhs: &i32) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == (*rhs as i64))
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, rhs: &i64) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == (*rhs as i64))
    }
}

impl PartialEq<isize> for Value {
    fn eq(&self, rhs: &isize) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == (*rhs as i64))
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, rhs: &u8) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == (*rhs as u64))
    }
}

impl PartialEq<u16> for Value {
    fn eq(&self, rhs: &u16) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == (*rhs as u64))
    }
}

impl PartialEq<u32> for Value {
    fn eq(&self, rhs: &u32) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == (*rhs as u64))
    }
}

impl PartialEq<u64> for Value {
    fn eq(&self, rhs: &u64) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == (*rhs as u64))
    }
}

impl PartialEq<usize> for Value {
    fn eq(&self, rhs: &usize) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == (*rhs as u64))
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, SeqAccess};

        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("any valid JSON API value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Number::from_f64(value).map_or(Value::Null, Value::Number))
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Value, E> {
                self.visit_string(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }

            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map = Map::with_capacity(access.size_hint().unwrap_or(0));

                while let Some(key) = access.next_key::<String>()? {
                    let key = key.parse().map_err(Error::custom)?;
                    let value = access.next_value()?;

                    map.insert(key, value);
                }

                Ok(Value::Object(map))
            }

            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut array = Vec::with_capacity(access.size_hint().unwrap_or(0));

                while let Some(value) = access.next_element()? {
                    array.push(value);
                }

                Ok(Value::Array(array))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Null => serializer.serialize_none(),
            Value::Array(ref value) => value.serialize(serializer),
            Value::Bool(value) => serializer.serialize_bool(value),
            Value::Number(ref value) => value.serialize(serializer),
            Value::Object(ref value) => value.serialize(serializer),
            Value::String(ref value) => serializer.serialize_str(value),
        }
    }
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    from_json(serde_json::to_value(value)?)
}

pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    Ok(T::deserialize(to_json(value))?)
}

fn to_json(value: Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Array(inner) => inner.into_iter().map(to_json).collect(),
        Value::Bool(inner) => JsonValue::Bool(inner),
        Value::Number(inner) => JsonValue::Number(inner),
        Value::Object(inner) => {
            let map = inner
                .into_iter()
                .map(|(k, v)| (String::from(k), to_json(v)))
                .collect();

            JsonValue::Object(map)
        }
        Value::String(inner) => JsonValue::String(inner),
    }
}

fn from_json(value: JsonValue) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(Value::Null),
        JsonValue::Array(data) => data.into_iter().map(from_json).collect(),
        JsonValue::Bool(data) => Ok(Value::Bool(data)),
        JsonValue::Number(data) => Ok(Value::Number(data)),
        JsonValue::Object(data) => data.into_iter()
            .map(|(k, v)| Ok((k.parse()?, from_json(v)?)))
            .collect(),
        JsonValue::String(data) => Ok(Value::String(data)),
    }
}
