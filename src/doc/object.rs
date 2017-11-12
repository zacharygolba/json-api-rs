use builder;
use doc::{Link, Relationship};
use error::Error;
use value::{Key, Map, Value};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Object {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub attributes: Map,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Key,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub relationships: Map<Key, Relationship>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Object {
    pub fn build() -> ObjectBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct ObjectBuilder {
    attributes: Vec<(String, Value)>,
    id: Option<String>,
    kind: Option<String>,
    links: Vec<(String, Link)>,
    meta: Vec<(String, Value)>,
    relationships: Vec<(String, Relationship)>,
}

impl ObjectBuilder {
    pub fn finalize(&mut self) -> Result<Object, Error> {
        let attributes = builder::map(&mut self.attributes, Ok)?;
        let id = builder::required("id", &mut self.id)?;
        let kind = builder::required("kind", &mut self.kind)?.parse()?;
        let links = builder::map(&mut self.links, Ok)?;
        let meta = builder::map(&mut self.meta, Ok)?;
        let relationships = builder::map(&mut self.relationships, Ok)?;

        Ok(Object {
            attributes,
            id,
            kind,
            links,
            meta,
            relationships,
            _ext: (),
        })
    }

    pub fn attribute<K>(&mut self, key: K, value: Value) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.attributes.push((key.as_ref().to_owned(), value));
        self
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

    pub fn relationship<K>(&mut self, key: K, value: Relationship) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.relationships.push((key.as_ref().to_owned(), value));
        self
    }
}
