//! Functions that convert types to and from a `Value`.

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::{self, Value as JsonValue};

use error::Error;
use value::Value;

/// Convert a `T` into a `Value`.
pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    from_json(serde_json::to_value(value)?)
}

/// Interpret a `Value` as an instance of type `T`.
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    Ok(T::deserialize(to_json(value))?)
}

pub(crate) fn to_json(value: Value) -> JsonValue {
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

pub(crate) fn from_json(value: JsonValue) -> Result<Value, Error> {
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
