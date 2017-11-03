use builder;
use doc::{Data, Identifier, Link};
use error::Error;
use value::{Key, Map, Value};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub data: Data<Identifier>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map<Key, Value>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Relationship {
    pub fn build() -> RelationshipBuilder {
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
    pub fn finalize(&mut self) -> Result<Relationship, Error> {
        let data = builder::required("data", &mut self.data)?;
        let links = builder::map(&mut self.links, Ok)?;
        let meta = builder::map(&mut self.meta, Ok)?;

        Ok(Relationship {
            data,
            links,
            meta,
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
