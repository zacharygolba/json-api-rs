pub mod error;
pub mod ident;
pub mod link;
pub mod object;
pub mod relationship;
pub mod specification;

use std::iter::FromIterator;

use serde::ser::Serialize;

use builder;
use sealed::Sealed;
use value::{Map, Value};

pub use self::error::Error;
pub use self::ident::Identifier;
pub use self::link::Link;
pub use self::object::Object;
pub use self::relationship::Relationship;
pub use self::specification::JsonApi;
#[doc(inline)]
pub use self::specification::Version;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Document<T: PrimaryData> {
    pub data: Data<T>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<Object>,
    #[serde(default)]
    pub jsonapi: JsonApi,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map<Value>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl<T: PrimaryData> Document<T> {
    pub fn build() -> DocumentBuilder<T> {
        Default::default()
    }
}

#[derive(Debug)]
pub struct DocumentBuilder<T: PrimaryData> {
    data: Option<Data<T>>,
    included: Vec<Object>,
    jsonapi: Option<JsonApi>,
    links: Vec<(String, Link)>,
    meta: Vec<(String, Value)>,
}

impl<T: PrimaryData> DocumentBuilder<T> {
    pub fn finalize(&mut self) -> Result<Document<T>, ::error::Error> {
        let data = builder::required("data", &mut self.data)?;
        let included = builder::iter(&mut self.included, Ok)?;
        let jsonapi = builder::default(&mut self.jsonapi);
        let links = builder::map(&mut self.links, Ok)?;
        let meta = builder::map(&mut self.meta, Ok)?;

        Ok(Document {
            data,
            included,
            jsonapi,
            links,
            meta,
            _ext: (),
        })
    }

    pub fn data<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Data<T>>,
    {
        self.data = Some(value.into());
        self
    }

    pub fn include(&mut self, value: Object) -> &mut Self {
        self.included.push(value);
        self
    }

    pub fn jsonapi(&mut self, value: JsonApi) -> &mut Self {
        self.jsonapi = Some(value);
        self
    }

    pub fn link<K>(&mut self, key: K, value: Link) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.links.push((key.as_ref().to_owned(), value));
        self
    }

    pub fn meta<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: Into<Value>,
    {
        self.meta.push((key.as_ref().to_owned(), value.into()));
        self
    }
}

impl<T: PrimaryData> Default for DocumentBuilder<T> {
    fn default() -> Self {
        DocumentBuilder {
            data: Default::default(),
            included: Default::default(),
            jsonapi: Default::default(),
            links: Default::default(),
            meta: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorDocument {
    pub errors: Vec<Error>,
    #[serde(default)]
    pub jsonapi: JsonApi,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map<Value>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl ErrorDocument {
    pub fn build() -> ErrorDocumentBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct ErrorDocumentBuilder {
    errors: Vec<Error>,
    included: Vec<Object>,
    jsonapi: Option<JsonApi>,
    links: Vec<(String, Link)>,
    meta: Vec<(String, Value)>,
}

impl ErrorDocumentBuilder {
    pub fn finalize(&mut self) -> Result<ErrorDocument, ::error::Error> {
        let errors = builder::iter(&mut self.errors, Ok)?;
        let jsonapi = builder::default(&mut self.jsonapi);
        let links = builder::map(&mut self.links, Ok)?;
        let meta = builder::map(&mut self.meta, Ok)?;

        Ok(ErrorDocument {
            errors,
            jsonapi,
            links,
            meta,
            _ext: (),
        })
    }

    pub fn error(&mut self, value: Error) -> &mut Self {
        self.errors.push(value);
        self
    }

    pub fn jsonapi(&mut self, value: JsonApi) -> &mut Self {
        self.jsonapi = Some(value);
        self
    }

    pub fn link<K>(&mut self, key: K, value: Link) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.links.push((key.as_ref().to_owned(), value));
        self
    }

    pub fn meta<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: Into<Value>,
    {
        self.meta.push((key.as_ref().to_owned(), value.into()));
        self
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Data<T: PrimaryData> {
    Collection(Vec<T>),
    Member(Box<Option<T>>),
}

impl<T: PrimaryData> From<Option<T>> for Data<T> {
    fn from(value: Option<T>) -> Self {
        Data::Member(Box::new(value))
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

pub trait PrimaryData: Sealed + Serialize {}

impl Sealed for Identifier {}
impl PrimaryData for Identifier {}

impl Sealed for Object {}
impl PrimaryData for Object {}
