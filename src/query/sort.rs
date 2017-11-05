use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Neg;
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

    /// Returns a cloned inverse of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// # use json_api::query::sort::{Direction, Sort};
    /// #
    /// # fn example() -> Result<(), Error> {
    /// let chrono = Sort::new("created-at".parse()?, Direction::Asc);
    /// let latest = chrono.reverse();
    ///
    /// assert_eq!(chrono.field, latest.field);
    /// assert_eq!(chrono.direction, Direction::Asc);
    /// assert_eq!(latest.direction, Direction::Desc);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn reverse(&self) -> Self {
        -self.clone()
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
            let field = (&value[1..]).parse()?;
            Ok(Sort::new(field, Direction::Desc))
        } else {
            let field = value.parse()?;
            Ok(Sort::new(field, Direction::Asc))
        }
    }
}

impl Neg for Sort {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Sort {
            direction: -self.direction,
            field: self.field,
            _ext: (),
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
    /// # use json_api::query::sort::Direction;
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
    /// # use json_api::query::sort::Direction;
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

    /// Returns a cloned inverse of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::query::sort::Direction;
    /// #
    /// # fn main() {
    /// let asc = Direction::Asc;
    /// let desc = Direction::Desc;
    ///
    /// assert_eq!(asc.reverse(), desc);
    /// assert_eq!(desc.reverse(), asc);
    /// # }
    /// ```
    pub fn reverse(&self) -> Self {
        -*self
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Asc => Direction::Desc,
            Direction::Desc => Direction::Asc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Sort};
    use value::key::Path;

    #[test]
    fn direction_is_asc() {
        assert_eq!(Direction::Asc.is_asc(), true);
        assert_eq!(Direction::Desc.is_asc(), false);
    }

    #[test]
    fn direction_is_desc() {
        assert_eq!(Direction::Desc.is_desc(), true);
        assert_eq!(Direction::Asc.is_desc(), false);
    }

    #[test]
    fn direction_reverse() {
        let asc = Direction::Asc;
        let desc = Direction::Desc;

        assert_eq!(asc.reverse(), desc);
        assert_eq!(desc.reverse(), asc);
    }

    #[test]
    fn sort_from_str() {
        let field = "created-at".parse::<Path>().unwrap();
        let mut sort = "created-at".parse::<Sort>().unwrap();

        assert_eq!(sort.field, field);
        assert_eq!(sort.direction, Direction::Asc);

        sort = "-created-at".parse().unwrap();

        assert_eq!(sort.field, field);
        assert_eq!(sort.direction, Direction::Desc);
    }
}
