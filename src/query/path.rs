use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;
use std::slice::Iter;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use error::Error;
use value::Key;

const SEPERATOR: u8 = '.' as u8;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path(Vec<Key>);

impl Path {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let Path(ref keys) = *self;
        let count = keys.len();

        if count > 0 {
            keys.iter()
                .map(|key| key.len())
                .fold(count - 1, |prev, next| prev + next)
        } else {
            count
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let capacity = self.len();
        let bytes = Vec::with_capacity(capacity);

        if capacity == 0 {
            return bytes;
        }

        self.iter().fold(bytes, |mut bytes, key| {
            if !bytes.is_empty() {
                bytes.push(SEPERATOR);
            }

            bytes.extend_from_slice(key.as_bytes());
            bytes
        })
    }

    pub fn to_string(&self) -> String {
        let bytes = self.to_bytes();
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Deref for Path {
    type Target = [Key];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Path> for String {
    fn from(path: Path) -> Self {
        path.to_string()
    }
}

impl From<Path> for Vec<u8> {
    fn from(path: Path) -> Self {
        path.to_bytes()
    }
}

impl FromIterator<Key> for Path {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Key>,
    {
        Path(Vec::from_iter(iter))
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.split('.').map(|item| item.parse()).collect()
    }
}

impl IntoIterator for Path {
    type Item = Key;
    type IntoIter = <Vec<Key> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a Key;
    type IntoIter = Iter<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};

        struct PathVisitor;

        impl<'de> Visitor<'de> for PathVisitor {
            type Value = Path;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(r#"a string of json api member names, separated by a ".""#)
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(PathVisitor)
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
