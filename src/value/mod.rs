pub mod key;
pub mod map;
pub mod set;

use std::fmt::{self, Formatter};
use std::iter::FromIterator;

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

impl From<bool> for Value {
    fn from(inner: bool) -> Self {
        Value::Bool(inner)
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<u16> for Value {
    fn from(n: u16) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Number(Number::from(n))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Value::Array(Vec::from_iter(iter))
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
