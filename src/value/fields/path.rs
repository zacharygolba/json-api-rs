use std::borrow::Borrow;
use std::fmt::{self, Display, Formatter};
use std::iter::{Extend, FromIterator};
use std::ops::Deref;
use std::slice::Iter;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use error::Error;
use sealed::Sealed;
use value::{Key, Stringify};

/// Represents a dot-separated list of member names.
///
/// See also: [relationship path].
///
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
        self.0.push(key);
    }

    /// Reserves capacity for at least `additional` more keys to be inserted.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows usize.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Path;
    /// #
    /// # fn main() {
    /// let mut path = Path::new();
    ///
    /// path.reserve(10);
    /// assert!(path.capacity() >= 10);
    /// # }
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Reserves the minimum capacity for exactly `additional` more keys to be
    /// inserted.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows usize.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Path;
    /// #
    /// # fn main() {
    /// let mut path = Path::new();
    ///
    /// path.reserve_exact(10);
    /// assert!(path.capacity() >= 10);
    /// # }
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }

    /// Shrinks the capacity of the path as much as possible.
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
    /// let mut path = Path::with_capacity(10);
    ///
    /// path.push("authors".parse()?);
    /// path.push("name".parse()?);
    ///
    /// path.shrink_to_fit();
    /// assert!(path.capacity() >= 2);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }
}

impl AsRef<[Key]> for Path {
    fn as_ref(&self) -> &[Key] {
        self
    }
}

impl Borrow<[Key]> for Path {
    fn borrow(&self) -> &[Key] {
        self
    }
}

impl Deref for Path {
    type Target = [Key];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.stringify())
    }
}

impl Extend<Key> for Path {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Key>,
    {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a Key> for Path {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a Key>,
    {
        self.extend(iter.into_iter().cloned());
    }
}

impl From<Path> for String {
    fn from(path: Path) -> Self {
        path.stringify()
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
        serializer.serialize_str(&self.stringify())
    }
}

impl Sealed for Path {}

impl Stringify for Path {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self.char_count() {
            0 => return Default::default(),
            len => Vec::with_capacity(len),
        };

        for key in self {
            if !bytes.is_empty() {
                bytes.push(b'.');
            }

            bytes.append(&mut key.to_bytes());
        }

        bytes
    }
}

/// Shared behavior for types that can be combined to create a `Path`.
pub trait Segment<T> {
    /// Combines `self` with `other`. Returns a new `Path`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// use json_api::value::fields::{Key, Path, Segment};
    ///
    /// let posts = "posts".parse::<Key>()?;
    /// let comments = "comments".parse::<Key>()?;
    ///
    /// let path: Path = posts.join(&comments);
    /// assert_eq!(path, "posts.comments");
    ///
    /// let authors = "authors".parse::<Key>()?;
    /// let name = "name".parse::<Key>()?;
    ///
    /// let path: Path = path.join(&authors.join(&name));
    /// assert_eq!(path, "posts.comments.authors.name");
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// # example().unwrap();
    /// # }
    /// ```
    fn join(&self, other: T) -> Path;
}

impl Segment<Key> for Key {
    fn join(&self, other: Key) -> Path {
        let mut path = Path::with_capacity(2);

        path.push(self.clone());
        path.push(other);

        path
    }
}

impl<'a> Segment<&'a Key> for Key {
    fn join(&self, other: &Key) -> Path {
        self.join(other.clone())
    }
}

impl<'a, T> Segment<T> for Key
where
    T: IntoIterator<Item = &'a Key>,
{
    fn join(&self, other: T) -> Path {
        let iter = other.into_iter();
        let mut path = match iter.size_hint() {
            (_, Some(size)) => Path::with_capacity(size + 1),
            _ => Path::new(),
        };

        path.push(self.clone());
        path.extend(iter);

        path
    }
}

impl Segment<Key> for Path {
    fn join(&self, other: Key) -> Path {
        let mut path = Path::with_capacity(self.len() + 1);

        path.extend(self);
        path.push(other);

        path
    }
}

impl<'a> Segment<&'a Key> for Path {
    fn join(&self, other: &Key) -> Path {
        self.join(other.clone())
    }
}

impl<'a, T> Segment<T> for Path
where
    T: IntoIterator<Item = &'a Key>,
{
    fn join(&self, other: T) -> Path {
        let iter = other.into_iter();
        let mut path = match iter.size_hint() {
            (_, Some(size)) => Path::with_capacity(self.len() + size),
            _ => Path::new(),
        };

        path.extend(self);
        path.extend(iter);

        path
    }
}
