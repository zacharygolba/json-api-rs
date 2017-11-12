use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;
use std::slice::Iter;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use error::Error;
use value::Key;

/// A sequence of `.` seperated JSON API [member names].
///
/// Commonly used in query params that accept a [relationship path].
///
/// [member names]: http://jsonapi.org/format/#document-member-names
/// [relationship path]: http://jsonapi.org/format/#fetching-includes
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path(Vec<Key>);

impl Path {
    /// Constructs a new, empty `Path`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Constructs a new, empty `Path` with the specified capacity.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let mut path = Path::with_capacity(2);
    ///
    /// path.push("a".parse()?);
    /// path.push("b".parse()?);
    ///
    /// // The next push will require reallocation...
    /// path.push("c".parse()?);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Path(Vec::with_capacity(capacity))
    }

    /// Returns the number of keys the path can hold without reallocating.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Path;
    /// #
    /// # fn main() {
    /// let path = Path::with_capacity(2);
    /// assert_eq!(path.capacity(), 2);
    /// # }
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of chars in a `Path`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use std::str::FromStr;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let path = Path::from_str("authors.name")?;
    ///
    /// assert_eq!(path.len(), 2);
    /// assert_eq!(path.char_count(), 12);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn char_count(&self) -> usize {
        let keys = &self.0;
        let count = keys.len();

        if count > 0 {
            keys.iter()
                .map(|key| key.len())
                .fold(count - 1, |prev, next| prev + next)
        } else {
            count
        }
    }

    /// Removes and returns a `Key` to the back of a `Path`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use std::str::FromStr;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let mut path = Path::from_str("authors.name")?;
    ///
    /// assert_eq!(path.pop(), Some("name".parse()?));
    /// assert_eq!(path.pop(), Some("authors".parse()?));
    /// assert_eq!(path.pop(), None);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn pop(&mut self) -> Option<Key> {
        self.0.pop()
    }

    /// Appends a `Key` to the back of a `Path`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let mut path = Path::new();
    ///
    /// path.push("authors".parse()?);
    /// path.push("name".parse()?);
    ///
    /// assert_eq!(path, "authors.name");
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn push(&mut self, key: Key) {
        self.0.push(key)
    }

    /// Converts the `Path` into an owned byte vector.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use std::str::FromStr;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let path = Path::from_str("a.b.c")?;
    /// assert_eq!(path.to_bytes(), vec![97, 46, 98, 46, 99]);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self.char_count() {
            0 => return Default::default(),
            len => Vec::with_capacity(len),
        };

        self.iter().fold(bytes, |mut bytes, key| {
            if !bytes.is_empty() {
                bytes.push(b'.');
            }

            bytes.extend_from_slice(key.as_bytes());
            bytes
        })
    }

    /// Converts the `Path` into an owned string.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use std::str::FromStr;
    /// #
    /// # use json_api::Error;
    /// # use json_api::value::Path;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let path = Path::from_str("a.b.c")?;
    /// assert_eq!(path.to_string(), "a.b.c".to_owned());
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn to_string(&self) -> String {
        let bytes = self.to_bytes();
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

impl Deref for Path {
    type Target = [Key];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Path {
    fn fmt(&self, fmtr: &mut Formatter) -> fmt::Result {
        fmtr.write_str(&self.to_string())
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
        value.split('.').map(Key::from_str).collect()
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

impl PartialEq<str> for Path {
    fn eq(&self, rhs: &str) -> bool {
        let mut parts = rhs.split('.');

        for part in self.iter().map(|key| Some(&**key)) {
            if part != parts.next() {
                return false;
            }
        }

        parts.next().is_none()
    }
}

impl<'a> PartialEq<&'a str> for Path {
    fn eq(&self, rhs: &&str) -> bool {
        self == *rhs
    }
}

impl PartialEq<String> for Path {
    fn eq(&self, rhs: &String) -> bool {
        self == &*rhs
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
