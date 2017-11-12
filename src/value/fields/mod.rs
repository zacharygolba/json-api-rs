//! Member names and field paths.

mod path;

use std::borrow::Borrow;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use error::Error;

pub use self::path::Path;

/// Represents a single member name.
///
/// When an instance of `Key` is parsed, the underlying value's casing
/// convention is converted to kebab-case.
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

    fn from_str(source: &str) -> Result<Key, Self::Err> {
        if source.is_empty() {
            bail!("cannot be blank");
        }

        // We should reserve a bit more than what we need so in
        // the event that we end up converting camelCase to
        // kebab-case, we don't have to reallocate.
        let mut dest = String::with_capacity(source.len() + 10);
        let mut chars = source.chars().peekable();

        while let Some(value) = chars.next() {
            match value {
                '\u{002e}' |
                '\u{002f}' |
                '\u{0040}' |
                '\u{0060}' |
                '\u{0000}'...'\u{001f}' |
                '\u{0021}'...'\u{0029}' |
                '\u{002a}'...'\u{002c}' |
                '\u{003a}'...'\u{003f}' |
                '\u{005b}'...'\u{005e}' |
                '\u{007b}'...'\u{007f}' => {
                    bail!("reserved '{}'", value);
                }
                '_' | '-' | ' ' if dest.is_empty() => {
                    bail!("cannot start with '{}'", value);
                }
                '_' | '-' | ' ' => match chars.peek() {
                    Some(&'-') | Some(&'_') | Some(&' ') | Some(&'A'...'Z') => {
                        continue;
                    }
                    Some(_) => {
                        dest.push('-');
                    }
                    None => {
                        bail!("cannot end with '{}'", value);
                    }
                },
                'A'...'Z' if dest.ends_with('-') => {
                    dest.push(as_lowercase(value));
                }
                'A'...'Z' => {
                    dest.push('-');
                    dest.push(as_lowercase(value));
                }
                _ => {
                    dest.push(value);
                }
            }
        }

        Ok(Key(dest))
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

#[inline]
fn as_lowercase(value: char) -> char {
    (value as u8 + 32) as char
}
