mod path;

use std::borrow::Borrow;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use inflector::Inflector;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use error::Error;

pub use self::path::Path;

/// A wrapper around `String` that enforces compliance with JSON API member names.
///
/// # Example
///
/// ```
/// # extern crate json_api;
/// #
/// # use std::str::FromStr;
/// #
/// # use json_api::Error;
/// # use json_api::value::Key;
/// #
/// # fn example() -> Result<(), Error> {
/// let key = Key::from_str("someFieldName")?;
/// assert_eq!(key, "some-field-name");
/// #
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example().unwrap()
/// # }
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Key(String);

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &**self
    }
}

impl Borrow<str> for Key {
    fn borrow(&self) -> &str {
        &**self
    }
}

impl Deref for Key {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        let Key(value) = key;
        value
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(value: &str) -> Result<Key, Self::Err> {
        if value.is_empty() {
            bail!("Member names cannot be blank");
        }

        let last = value.len() - 1;
        for (idx, chr) in value.chars().enumerate() {
            match chr {
                '\u{002b}' |
                '\u{002c}' |
                '\u{002e}' |
                '\u{005b}' |
                '\u{005d}' |
                '\u{0021}' |
                '\u{0022}' |
                '\u{0023}' |
                '\u{0024}' |
                '\u{0025}' |
                '\u{0026}' |
                '\u{0027}' |
                '\u{0028}' |
                '\u{0029}' |
                '\u{002a}' |
                '\u{002f}' |
                '\u{003a}' |
                '\u{003b}' |
                '\u{003c}' |
                '\u{003d}' |
                '\u{003e}' |
                '\u{003f}' |
                '\u{0040}' |
                '\u{005c}' |
                '\u{005e}' |
                '\u{0060}' |
                '\u{007b}' |
                '\u{007c}' |
                '\u{007d}' |
                '\u{007e}' |
                '\u{007f}' |
                '\u{0000}'...'\u{001f}' => {
                    bail!("Member names cannot contain {}", chr);
                }
                '\u{002d}' | '\u{005f}' | '\u{0020}' if idx == 0 => {
                    bail!("Member names cannot start with {}", chr);
                }
                '\u{002d}' | '\u{005f}' | '\u{0020}' if idx == last => {
                    bail!("Member names cannot end with {}", chr);
                }
                _ => (),
            }
        }

        Ok(Key(value.to_kebab_case()))
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, rhs: &String) -> bool {
        &self.0 == rhs
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, rhs: &str) -> bool {
        &**self == rhs
    }
}

impl<'a> PartialEq<&'a str> for Key {
    fn eq(&self, rhs: &&str) -> bool {
        &**self == *rhs
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Key, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("a valid json api member name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value.parse().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(KeyVisitor)
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}
