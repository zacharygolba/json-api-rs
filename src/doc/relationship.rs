use builder;
use doc::{Data, Identifier, Link};
use error::Error;
use value::{Key, Map, Value};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Relationship {
    pub data: Data<Identifier>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Relationship {
    pub fn builder() -> RelationshipBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct RelationshipBuilder {
    data: Option<Data<Identifier>>,
    links: Vec<(String, Link)>,
    meta: Vec<(String, Value)>,
}

impl RelationshipBuilder {
    pub fn build(&mut self) -> Result<Relationship, Error> {
        Ok(Relationship {
            data: builder::required("data", &mut self.data)?,
            links: builder::iter(&mut self.links, builder::parse_key)?,
            meta: builder::iter(&mut self.meta, builder::parse_key)?,
            _ext: (),
        })
    }

    pub fn data(&mut self, value: Data<Identifier>) -> &mut Self {
        self.data = Some(value);
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
