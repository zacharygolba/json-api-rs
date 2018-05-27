//! An API for working with well-known query parameters.

mod builder;
mod page;
mod sort;

use std::fmt::{self, Formatter};

use percent_encoding::percent_decode;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_qs;

use error::Error;
use value::{Key, Map, Path, Set, Value};

pub use self::builder::Builder;
pub use self::page::Page;
pub use self::sort::{Direction, Sort};

/// Represents well-known query parameters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    /// A map where each key is a type name and the value is set of field names
    /// that the client wishes to receive for the given type. If this is not present
    /// when decoding a query string, an empty map is used (no allocation is required).
    ///
    /// It is recommeneded that consumers of this crate interpret a `None` value as the
    /// client specifying that they want all of the available fields for a given type.
    /// This behavior is already implemented when rendering a type that implements the
    /// [`Resource`] trait via the [`resource!`] macro.
    ///
    /// For more information, check out the *[sparse fieldsets]* section of the JSON API
    /// specification.
    ///
    /// [`Resource`]: ../trait.Resource.html
    /// [`resource!`]: ../macro.resource.html
    /// [sparse fieldsets]: http://jsonapi.org/format/#fetching-sparse-fieldsets
    pub fields: Map<Key, Set>,

    /// A map where each key is a field path and the value is the value the client
    /// would like each item in the return document to have for the given field.
    ///
    /// For more information, check out the *[filter]* section of the JSON API
    /// specification.
    ///
    /// [filtering]: http://jsonapi.org/format/#fetching-filtering
    pub filter: Map<Path, Value>,

    /// A set of relationship paths that specify included resources a client wishes to
    /// receive in addition to a document's primary data.
    ///
    /// For more information, check out the *[inclusion of related resources]* section
    /// of the JSON API specification.
    ///
    /// [inclusion of related resources]: http://jsonapi.org/format/#fetching-includes
    pub include: Set<Path>,

    /// Optional pagination parameters. To make life easier when this value is `None`,
    /// the `Page` struct implements a sensible default.
    ///
    /// For more information, check out the *[pagination]* section of the JSON API
    /// specification.
    ///
    /// [pagination]: http://jsonapi.org/format/#fetching-pagination
    pub page: Option<Page>,

    /// A set of sort instructions. Each element in the set contains the field name, and
    /// the sort direction (ascending or descending).
    ///
    /// When a client specifies a field to sort by, if it is prefixed with `'-'` (i.e
    /// `sort=-created-at`) the instruction is interpreted to mean "sort by the field
    /// 'created-at' in descending order".
    ///
    /// For more information, check out the *[sorting]* section of the JSON API
    /// specification.
    ///
    /// [sorting]: http://jsonapi.org/format/#fetching-sorting
    pub sort: Set<Sort>,

    /// Private field for backwards compatibility.
    _ext: (),
}

impl Query {
    /// Returns the decoded equivalent of an empty query string. Does not
    /// require any allocations.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a query builder that can be used to create a new query.
    pub fn builder() -> Builder {
        Default::default()
    }
}

impl<'de> Deserialize<'de> for Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fields", "filter", "include", "page", "sort"];

        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Fields,
            Filter,
            Include,
            Page,
            Sort,
        }

        struct QueryVisitor;

        impl<'de> Visitor<'de> for QueryVisitor {
            type Value = Query;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an object containing query parameters")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                let mut fields = None;
                let mut filter = None;
                let mut include = None;
                let mut page = None;
                let mut sort = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        Field::Fields => {
                            let data = access.next_value::<Map<_, String>>()?;
                            let mut map = Map::with_capacity(data.len());

                            for (field, value) in data {
                                let value = value.parse().map_err(Error::custom)?;
                                map.insert(field, value);
                            }

                            fields = Some(map);
                        }
                        Field::Filter => {
                            filter = Some(access.next_value()?);
                        }
                        Field::Include => {
                            let data = access.next_value::<String>()?;
                            include = Some(data.parse().map_err(Error::custom)?);
                        }
                        Field::Page => {
                            page = Some(access.next_value()?);
                        }
                        Field::Sort => {
                            let data = access.next_value::<String>()?;
                            sort = Some(data.parse().map_err(Error::custom)?);
                        }
                    }
                }

                Ok(Query {
                    page,
                    fields: fields.unwrap_or_default(),
                    filter: filter.unwrap_or_default(),
                    include: include.unwrap_or_default(),
                    sort: sort.unwrap_or_default(),
                    _ext: (),
                })
            }
        }

        deserializer.deserialize_struct("Query", FIELDS, QueryVisitor)
    }
}

impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Query", 5)?;

        if !self.fields.is_empty() {
            let mut fields = Map::with_capacity(self.fields.len());

            for (key, value) in &self.fields {
                fields.insert(key, value.to_string());
            }

            state.serialize_field("fields", &fields)?;
        }

        if !self.filter.is_empty() {
            state.serialize_field("filter", &self.filter)?;
        }

        if !self.include.is_empty() {
            state.serialize_field("include", &self.include.to_string())?;
        }

        if let Some(ref page) = self.page {
            state.serialize_field("page", page)?;
        }

        if !self.sort.is_empty() {
            state.serialize_field("sort", &self.sort.to_string())?;
        }

        state.end()
    }
}

/// Deserialize a `Query` from the bytes of a percent encoded query string.
pub fn from_slice(data: &[u8]) -> Result<Query, Error> {
    let value = percent_decode(data).decode_utf8()?;
    Ok(serde_qs::from_bytes(value.as_bytes())?)
}

/// Deserialize a `Query` from a percent encoded query string.
pub fn from_str(data: &str) -> Result<Query, Error> {
    from_slice(data.as_bytes())
}

/// Serialize the given `Query` as a percent encoded query string.
pub fn to_string(query: &Query) -> Result<String, Error> {
    use percent_encoding::{percent_encode, QUERY_ENCODE_SET};

    let value = serde_qs::to_string(query)?;
    let data = value.as_bytes();

    Ok(percent_encode(data, QUERY_ENCODE_SET).collect())
}

/// Serialize the given `Query` as a representing percent encoded query string
/// vector of bytes.
pub fn to_vec(query: &Query) -> Result<Vec<u8>, Error> {
    to_string(query).map(Vec::from)
}
