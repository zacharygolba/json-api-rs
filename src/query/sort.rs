use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use error::Error;
use query::Path;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Sort {
    pub direction: Direction,
    pub field: Path,
    /// Private field for backwards compatibility.
    _ext: (),
}

impl Sort {
    pub fn new(field: Path, direction: Direction) -> Self {
        Sort {
            direction,
            field,
            _ext: (),
        }
    }
}

impl Debug for Sort {
    fn fmt(&self, fmtr: &mut Formatter) -> fmt::Result {
        fmtr.debug_struct("Sort")
            .field("direction", &self.direction)
            .field("field", &self.field)
            .finish()
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.direction == Direction::Desc {
            Display::fmt("-", f)?;
        }

        Display::fmt(&self.field, f)
    }
}

impl FromStr for Sort {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with('-') {
            Ok(Sort {
                direction: Direction::Desc,
                field: (&value[1..]).parse()?,
                _ext: (),
            })
        } else {
            Ok(Sort {
                direction: Direction::Asc,
                field: value.parse()?,
                _ext: (),
            })
        }
    }
}

impl<'de> Deserialize<'de> for Sort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};

        struct SortVisitor;

        impl<'de> Visitor<'de> for SortVisitor {
            type Value = Sort;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "a valid json api member name, optionally ")?;
                write!(f, r#"prefixed with "-" to denote descending order"#)
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(SortVisitor)
    }
}

impl Serialize for Sort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut value = String::with_capacity(self.field.len() + 1);

        if self.direction == Direction::Desc {
            value.push('-');
        }

        value.push_str(&self.field.to_string());
        value.serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

impl Direction {
    /// Returns `true` if the direction is [`Asc`].
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::query::Direction;
    /// #
    /// # fn main() {
    /// let direction = Direction::Desc;
    /// assert_eq!(direction.is_asc(), false);
    ///
    /// let direction = Direction::Asc;
    /// assert_eq!(direction.is_asc(), true);
    /// # }
    /// ```
    ///
    /// [`Asc`]: #variant.Asc
    pub fn is_asc(&self) -> bool {
        *self == Direction::Asc
    }

    /// Returns `true` if the direction is [`Desc`].
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::query::Direction;
    /// #
    /// # fn main() {
    /// let direction = Direction::Asc;
    /// assert_eq!(direction.is_desc(), false);
    ///
    /// let direction = Direction::Desc;
    /// assert_eq!(direction.is_desc(), true);
    /// # }
    /// ```
    ///
    /// [`Desc`]: #variant.Desc
    pub fn is_desc(&self) -> bool {
        *self == Direction::Desc
    }
}
