//! Provides types that can be used to represent valid JSON API data.

pub(crate) mod convert;

pub mod collections;
pub mod fields;

use std::cmp::PartialEq;
use std::fmt::{self, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use error::Error;

pub use serde_json::value::Number;

pub use self::collections::{Map, Set};
pub use self::convert::{from_value, to_value};
#[doc(no_inline)]
pub use self::fields::{Key, Path};

/// Represents any valid JSON API value.
///
/// Like [`serde_json::Value`], but with spec-compliance baked into the type
/// system.
///
/// [`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Represents a null JSON value.
    Null,
    /// Represents an array of values.
    Array(Vec<Value>),
    /// Represents a boolean value (true and false).
    Bool(bool),
    /// Represents both integers and floating point values.
    Number(Number),
    /// Represents a JSON object as a hash table with consistent order. Keys are
    /// guarenteed to be a valid member name.
    Object(Map),
    /// Represents a JSON string.
    String(String),
}

impl Value {
    /// Optionally get the underlying vector as a slice. Returns `None` if the
    /// `Value` is not an array.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let data = vec![true.into(), false.into()];
    /// let array = Value::Array(data.clone());
    /// let boolean = Value::Bool(true);
    ///
    /// assert_eq!(array.as_array(), Some(data.as_slice()));
    /// assert_eq!(boolean.as_array(), None);
    /// # }
    /// ```
    pub fn as_array(&self) -> Option<&[Value]> {
        match *self {
            Value::Array(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Optionally get the underlying vector as a mutable slice. Returns `None`
    /// if the `Value` is not an array.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let mut data = vec![true.into(), false.into()];
    /// let mut array = Value::Array(data.clone());
    /// let mut boolean = Value::Bool(true);
    ///
    /// assert_eq!(array.as_array_mut(), Some(data.as_mut_slice()));
    /// assert_eq!(boolean.as_array_mut(), None);
    /// # }
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut [Value]> {
        match *self {
            Value::Array(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    /// Optionally get the inner boolean value. Returns `None` if the `Value` is
    /// not a boolean.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let boolean = Value::Bool(true);
    /// let number = Value::from(3.14);
    ///
    /// assert_eq!(boolean.as_bool(), Some(true));
    /// assert_eq!(number.as_bool(), None);
    /// # }
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns `Some(())` if the `Value` is null.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let null = Value::Null;
    /// let text = Value::String("Hello, World!".to_owned());
    ///
    /// assert_eq!(null.as_null(), Some(()));
    /// assert_eq!(text.as_null(), None);
    /// # }
    /// ```
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    /// Optionally get a reference to the inner map. Returns `None` if the
    /// `Value` is not an object.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::{Map, Value};
    /// #
    /// # fn main() {
    /// let data = Map::new();
    /// let object = Value::Object(data.clone());
    /// let number = Value::from(3.14);
    ///
    /// assert_eq!(object.as_object(), Some(&data));
    /// assert_eq!(number.as_object(), None);
    /// # }
    /// ```
    pub fn as_object(&self) -> Option<&Map> {
        match *self {
            Value::Object(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Optionally get a mutable reference to the inner map. Returns `None` if
    /// the `Value` is not an object.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::{Map, Value};
    /// #
    /// # fn main() {
    /// let mut data = Map::new();
    /// let mut object = Value::Object(data.clone());
    /// let mut number = Value::from(3.14);
    ///
    /// assert_eq!(object.as_object_mut(), Some(&mut data));
    /// assert_eq!(number.as_object_mut(), None);
    /// # }
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut Map> {
        match *self {
            Value::Object(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    /// Optionally get the underlying string as a string slice. Returns `None`
    /// if the `Value` is not a string.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let data = "Hello, World!";
    /// let string = Value::String(data.to_owned());
    /// let number = Value::from(3.14);
    ///
    /// assert_eq!(string.as_str(), Some(data));
    /// assert_eq!(number.as_str(), None);
    /// # }
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Optionally get the underlying number as an `f64`. Returns `None` if the
    /// `Value` cannot be represented as an `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let number = Value::from(3.14);
    /// let string = Value::String("Hello, World!".to_owned());
    ///
    /// assert_eq!(number.as_f64(), Some(3.14));
    /// assert_eq!(string.as_f64(), None);
    /// # }
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }

    /// Optionally get the underlying number as an `i64`. Returns `None` if the
    /// `Value` cannot be represented as an `i64`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let integer = Value::from(10);
    /// let float = Value::from(3.14);
    ///
    /// assert_eq!(integer.as_i64(), Some(10));
    /// assert_eq!(float.as_i64(), None);
    /// # }
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }

    /// Optionally get the underlying number as an `u64`. Returns `None` if the
    /// `Value` cannot be represented as an `u64`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let positive = Value::from(10);
    /// let negative = Value::from(-10);
    ///
    /// assert_eq!(positive.as_u64(), Some(10));
    /// assert_eq!(negative.as_u64(), None);
    /// # }
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an array.
    ///
    /// For any `Value` on which `is_array` returns true, [`as_array`] and
    /// [`as_array_mut`] are guaranteed to return a reference to the vector
    /// representing the array.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let mut value = Value::from(vec![1, 2, 3]);
    ///
    /// assert!(value.is_array());
    ///
    /// value.as_array().unwrap();
    /// value.as_array_mut().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_array`]: #method.as_array
    /// [`as_array_mut`]: #method.as_array_mut
    pub fn is_array(&self) -> bool {
        match *self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a boolean.
    ///
    /// For any `Value` on which `is_boolean` returns true, [`as_bool`] is
    /// guaranteed to return the boolean value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let value = Value::Bool(true);
    ///
    /// assert!(value.is_boolean());
    /// value.as_bool().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_bool`]: #method.as_bool
    pub fn is_boolean(&self) -> bool {
        match *self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is null.
    ///
    /// For any `Value` on which `is_null` returns true, [`as_null`] is
    /// guaranteed to return `Some(())`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let value = Value::Null;
    ///
    /// assert!(value.is_null());
    /// value.as_null().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_null`]: #method.as_null
    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a number.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// assert!(Value::from(3.14).is_number());
    /// # }
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is an object.
    ///
    /// For any `Value` on which `is_array` returns true, [`as_object`] and
    /// [`as_object_mut`] are guaranteed to return a reference to the map
    /// representing the object.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let mut value = Value::Object(Default::default());
    ///
    /// assert!(value.is_object());
    ///
    /// value.as_object().unwrap();
    /// value.as_object_mut().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_object`]: #method.as_object
    /// [`as_object_mut`]: #method.as_object_mut
    pub fn is_object(&self) -> bool {
        match *self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a string.
    ///
    /// For any `Value` on which `is_string` returns true, [`as_str`] is
    /// guaranteed to return the string slice.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let value = Value::String("Hello, world!".to_owned());
    ///
    /// assert!(value.is_string());
    /// value.as_str().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_str`]: #method.as_str
    pub fn is_string(&self) -> bool {
        match *self {
            Value::String(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a number that can be represented as an
    /// `f64`.
    ///
    /// For any `Value` on which `is_f64` returns true, [`as_f64`] is
    /// guaranteed to return the floating point value.
    ///
    /// Currently this function returns true if and only if both [`is_i64`] and
    /// [`is_u64`] return false. This behavior is not a guarantee in the future.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let value = Value::from(3.14);
    ///
    /// assert!(value.is_f64());
    /// value.as_f64().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_f64`]: #method.as_f64
    /// [`is_i64`]: #method.is_i64
    /// [`is_u64`]: #method.is_u64
    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Value on which `is_i64` returns true, [`as_i64`] is guaranteed
    /// to return the integer value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let pos = Value::from(3);
    /// let neg = Value::from(-3);
    ///
    /// assert!(pos.is_i64());
    /// assert!(neg.is_i64());
    ///
    /// pos.as_i64().unwrap();
    /// neg.as_i64().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_i64`]: #method.as_i64
    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between `0` and `u64::MAX`.
    ///
    /// For any Value on which `is_u64` returns true, [`as_u64`] is guaranteed
    /// to return the integer value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Value;
    /// #
    /// # fn main() {
    /// let value = Value::from(3);
    ///
    /// assert!(value.is_u64());
    /// value.as_u64().unwrap();
    /// # }
    /// ```
    ///
    /// [`as_u64`]: #method.as_u64
    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }
}

/// Returns the `Value::Null`. This allows for better composition with `Option`
/// types.
///
/// # Example
///
/// ```
/// # extern crate json_api;
/// #
/// # use json_api::Value;
/// #
/// # fn main() {
/// const MSG: &'static str = "Hello, World!";
///
/// let opt = None;
/// let value = opt.map(Value::String).unwrap_or_default();
/// assert_eq!(value, Value::Null);
///
/// let opt = Some(MSG.to_owned());
/// let value = opt.map(Value::String).unwrap_or_default();
/// assert_eq!(value, Value::String(MSG.to_owned()));
/// # }
/// ```
impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(inner: bool) -> Self {
        Value::Bool(inner)
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Self {
        Value::from(f64::from(n))
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Number::from_f64(n).map(Value::Number).unwrap_or_default()
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        Value::from(i64::from(n))
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        Value::from(i64::from(n))
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::from(i64::from(n))
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n.into())
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Self {
        Value::from(u64::from(n))
    }
}

impl From<u16> for Value {
    fn from(n: u16) -> Self {
        Value::from(u64::from(n))
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::from(u64::from(n))
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

impl From<Map> for Value {
    fn from(data: Map) -> Self {
        Value::Object(data)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(data: Option<T>) -> Self {
        data.map(T::into).unwrap_or_default()
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
        convert::from_json(src.parse()?)
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, rhs: &bool) -> bool {
        self.as_bool().map_or(false, |lhs| lhs == *rhs)
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, rhs: &f32) -> bool {
        *self == f64::from(*rhs)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, rhs: &f64) -> bool {
        self.as_f64().map_or(false, |lhs| lhs == *rhs)
    }
}

impl PartialEq<i8> for Value {
    fn eq(&self, rhs: &i8) -> bool {
        *self == i64::from(*rhs)
    }
}

impl PartialEq<i16> for Value {
    fn eq(&self, rhs: &i16) -> bool {
        *self == i64::from(*rhs)
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, rhs: &i32) -> bool {
        *self == i64::from(*rhs)
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, rhs: &i64) -> bool {
        self.as_i64().map_or(false, |lhs| lhs == *rhs)
    }
}

impl PartialEq<isize> for Value {
    fn eq(&self, rhs: &isize) -> bool {
        *self == (*rhs as i64)
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, rhs: &u8) -> bool {
        *self == u64::from(*rhs)
    }
}

impl PartialEq<u16> for Value {
    fn eq(&self, rhs: &u16) -> bool {
        *self == u64::from(*rhs)
    }
}

impl PartialEq<u32> for Value {
    fn eq(&self, rhs: &u32) -> bool {
        *self == u64::from(*rhs)
    }
}

impl PartialEq<u64> for Value {
    fn eq(&self, rhs: &u64) -> bool {
        self.as_u64().map_or(false, |lhs| lhs == *rhs)
    }
}

impl PartialEq<usize> for Value {
    fn eq(&self, rhs: &usize) -> bool {
        *self == (*rhs as u64)
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

            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
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
