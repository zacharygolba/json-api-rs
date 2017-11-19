//! Components of a JSON API document.

mod convert;
mod ident;
mod link;
mod object;
mod relationship;
mod specification;

mod error;

use std::iter::FromIterator;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use sealed::Sealed;
use value::{Key, Map, Set, Value};

pub use self::convert::*;
pub use self::error::{ErrorObject, ErrorSource};
pub use self::ident::Identifier;
pub use self::link::Link;
pub use self::object::{NewObject, Object};
pub use self::relationship::Relationship;
pub use self::specification::{JsonApi, Version};

/// A marker trait used to indicate that a type can be the primary data for a
/// document.
pub trait PrimaryData: DeserializeOwned + Sealed + Serialize {
    #[doc(hidden)]
    fn flatten(self, &Set<Object>) -> Value;
}

/// Represents a compound JSON API document.
///
/// For more information, check out the *[document structure]* section of the JSON API
/// specification.
///
/// [document structure]: https://goo.gl/CXTNmt
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(bound = "T: PrimaryData", untagged)]
pub enum Document<T: PrimaryData> {
    /// Does not contain errors.
    Ok {
        /// The primary data of the document. For more information, check out the
        /// *[top level]* section of the JSON API specification.
        ///
        /// [top level]: https://goo.gl/fQdYgo
        data: Data<T>,

        /// Included resources, resolved from the `include` query parameter of a client
        /// request.
        #[serde(default, skip_serializing_if = "Set::is_empty")]
        included: Set<Object>,

        /// Information about this implementation of the specification that the
        /// document was created with. For more information, check out the *[JSON API
        /// object]* section of the JSON API specification.
        ///
        /// [JSON API object]: https://goo.gl/hZUcEt
        #[serde(default)]
        jsonapi: JsonApi,

        /// Contains relevant links. If this value of this field is empty, it will not be
        /// serialized. For more information, check out the *[links]* section of the JSON
        /// API specification.
        ///
        /// [links]: https://goo.gl/E4E6Vt
        #[serde(default, skip_serializing_if = "Map::is_empty")]
        links: Map<Key, Link>,

        /// Non-standard meta information. If this value of this field is empty, it will
        /// not be serialized. For more information, check out the *[meta
        /// information]* section of the JSON API specification.
        ///
        /// [meta information]: https://goo.gl/LyrGF8
        #[serde(default, skip_serializing_if = "Map::is_empty")]
        meta: Map,
    },

    /// Contains 1 or more error(s).
    Err {
        errors: Vec<ErrorObject>,

        #[serde(default)]
        jsonapi: JsonApi,

        #[serde(default, skip_serializing_if = "Map::is_empty")]
        links: Map<Key, Link>,

        #[serde(default, skip_serializing_if = "Map::is_empty")]
        meta: Map,
    },
}

impl<T: PrimaryData> Document<T> {
    /// Returns `true` if the document does not contain any errors.
    pub fn is_ok(&self) -> bool {
        match *self {
            Document::Ok { .. } => true,
            Document::Err { .. } => false,
        }
    }

    /// Returns `true` if the document contains 1 or more error(s).
    pub fn is_err(&self) -> bool {
        match *self {
            Document::Ok { .. } => true,
            Document::Err { .. } => false,
        }
    }
}

/// Describes the data of a document or resource linkage.
///
/// For more information, check out the *[top level]* section of the JSON API
/// specification.
///
/// [top level]: https://goo.gl/fQdYgo
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(bound = "T: PrimaryData", untagged)]
pub enum Data<T: PrimaryData> {
    /// A collection of `T`. Used for requests that target resource collections.
    Collection(Vec<T>),

    /// An optional `T`. Used for requests that target single resources.
    Member(Box<Option<T>>),
}

impl<T: PrimaryData> From<Option<T>> for Data<T> {
    fn from(value: Option<T>) -> Self {
        Data::Member(Box::new(value))
    }
}

impl<T: PrimaryData> From<Vec<T>> for Data<T> {
    fn from(value: Vec<T>) -> Self {
        Data::Collection(value)
    }
}

impl<T: PrimaryData> From<T> for Data<T> {
    fn from(value: T) -> Self {
        Data::Member(Box::new(Some(value)))
    }
}

impl<T: PrimaryData> FromIterator<T> for Data<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Data::Collection(Vec::from_iter(iter))
    }
}
