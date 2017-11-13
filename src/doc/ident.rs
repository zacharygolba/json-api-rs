use std::cmp::PartialEq;
use std::mem;

use builder;
use doc::{Data, Document, IntoDocument, Object, PrimaryData};
use error::Error;
use sealed::Sealed;
use value::{Key, Map, Value};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Identifier {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Key,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Identifier {
    pub fn builder() -> IdentifierBuilder {
        Default::default()
    }
}

impl IntoDocument<Identifier> for Identifier {
    fn to_doc(mut self) -> Result<Document<Identifier>, Error> {
        let meta = mem::replace(&mut self.meta, Default::default());
        let mut doc = Document::new(Data::Member(Box::new(Some(self))));

        doc.meta = meta;
        Ok(doc)
    }
}

impl PartialEq<Object> for Identifier {
    fn eq(&self, rhs: &Object) -> bool {
        self.id == rhs.id && self.kind == rhs.kind
    }
}

impl<'a> PartialEq<&'a Object> for Identifier {
    fn eq(&self, rhs: &&'a Object) -> bool {
        *self == **rhs
    }
}

impl PrimaryData for Identifier {
    fn flatten(self, incl: &[Object]) -> Value {
        incl.into_iter()
            .find(|item| self == *item)
            .map(|item| item.clone().flatten(incl))
            .unwrap_or_default()
    }
}

impl Sealed for Identifier {}

#[derive(Debug, Default)]
pub struct IdentifierBuilder {
    id: Option<String>,
    kind: Option<String>,
    meta: Vec<(String, Value)>,
}

impl IdentifierBuilder {
    pub fn build(&mut self) -> Result<Identifier, Error> {
        Ok(Identifier {
            id: builder::required("id", &mut self.id)?,
            kind: builder::required("kind", &mut self.kind)?.parse()?,
            meta: builder::iter(&mut self.meta, builder::parse_key)?,
            _ext: (),
        })
    }

    pub fn id<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.id = Some(value.as_ref().to_owned());
        self
    }

    pub fn kind<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.kind = Some(value.as_ref().to_owned());
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
