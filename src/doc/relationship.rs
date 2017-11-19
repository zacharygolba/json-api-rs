use std::iter::FromIterator;

use doc::{Data, Identifier, Link};
use value::{Key, Map};

/// Represents a resource's relationship to another.
///
/// For more information, check out the *[relationships]* section of the JSON API
/// specification.
///
/// [relationships]: https://goo.gl/ZQw9Xr
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Relationship {
    /// Contains resource linkage. For more information, checkout the
    /// *[resource linkage]* section of the JSON API specification.
    ///
    /// [resource linkage]: https://goo.gl/evZF8m
    pub data: Data<Identifier>,

    /// Contains relevant links. If this value of this field is empty, it will not be
    /// serialized. For more information, check out the *[links]* section of the JSON
    /// API specification.
    ///
    /// [links]: https://goo.gl/E4E6Vt
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,

    /// Non-standard meta information. If this value of this field is empty, it will not
    /// be serialized. For more information, check out the *[meta information]* section
    /// of the JSON API specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,

    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Relationship {
    /// Returns a new `Relationship`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// use json_api::doc::{Data, Identifier, Relationship};
    ///
    /// let ident = Identifier::new("users".parse()?, "1".to_owned());
    /// let data = Data::Member(Box::new(Some(ident)));
    /// let mut relationship = Relationship::new(data);
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// # example().unwrap();
    /// # }
    /// ```
    pub fn new(data: Data<Identifier>) -> Self {
        Relationship {
            data,
            links: Default::default(),
            meta: Default::default(),
            _ext: (),
        }
    }
}

impl From<Option<Identifier>> for Relationship {
    fn from(value: Option<Identifier>) -> Self {
        let data = Data::Member(Box::new(value));
        Relationship::new(data)
    }
}

impl From<Vec<Identifier>> for Relationship {
    fn from(value: Vec<Identifier>) -> Self {
        let data = Data::Collection(value);
        Relationship::new(data)
    }
}

impl From<Identifier> for Relationship {
    fn from(value: Identifier) -> Self {
        Relationship::from(Some(value))
    }
}

impl FromIterator<Identifier> for Relationship {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Identifier>,
    {
        let data = Data::from_iter(iter);
        Relationship::new(data)
    }
}
