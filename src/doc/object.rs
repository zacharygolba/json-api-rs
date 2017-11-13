use std::mem;

use builder;
use doc::{Data, Document, Identifier, IntoDocument, Link, PrimaryData, Relationship};
use error::Error;
use sealed::Sealed;
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
    pub fn builder() -> ObjectBuilder {
        Default::default()
    }
}

impl IntoDocument<Identifier> for Object {
    fn to_doc(self) -> Result<Document<Identifier>, Error> {
        let mut data = Identifier::builder().id(&self.id).kind(&self.kind).build()?;

        data.meta.extend(self.meta);
        data.to_doc()
    }
}

impl IntoDocument<Object> for Object {
    fn to_doc(mut self) -> Result<Document<Object>, Error> {
        let links = mem::replace(&mut self.links, Default::default());
        let meta = mem::replace(&mut self.meta, Default::default());

        let mut doc = Document::new(Data::Member(Box::new(Some(self))));
        doc.links = links;
        doc.meta = meta;

        Ok(doc)
    }
}

impl PrimaryData for Object {
    fn flatten(self, incl: &[Object]) -> Value {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let Object { id, attributes, relationships, .. } = self;
        let mut map = {
            let size = attributes.len() + relationships.len() + 1;
            Map::with_capacity(size)
        };

        map.insert(Key::from_raw("id".to_owned()), Value::String(id));
        map.extend(attributes);

        for (key, value) in relationships {
            let value = match value.data {
                Data::Member(data) => match *data {
                    Some(item) => item.flatten(incl),
                    None => Value::Null,
                },
                Data::Collection(data) => {
                    let iter = data.into_iter().map(|item| item.flatten(incl));
                    Value::Array(iter.collect())
                }
            };

            map.insert(key, value);
        }

        Value::Object(map)
    }
}

impl Sealed for Object {}

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
    pub fn build(&mut self) -> Result<Object, Error> {
        Ok(Object {
            attributes: builder::iter(&mut self.attributes, builder::parse_key)?,
            id: builder::required("id", &mut self.id)?,
            kind: builder::required("kind", &mut self.kind)?.parse()?,
            links: builder::iter(&mut self.links, builder::parse_key)?,
            meta: builder::iter(&mut self.meta, builder::parse_key)?,
            relationships: builder::iter(&mut self.relationships, builder::parse_key)?,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NewObject {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub attributes: Map,
    pub id: Option<String>,
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

impl PrimaryData for NewObject {
    fn flatten(self, _: &[Object]) -> Value {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let NewObject { id, attributes, relationships, .. } = self;
        let mut map = {
            let size = attributes.len() + relationships.len() + 1;
            Map::with_capacity(size)
        };

        if let Some(value) = id {
            map.insert(Key::from_raw("id".to_owned()), Value::String(value));
        }

        map.extend(attributes);

        for (key, value) in relationships {
            let value = match value.data {
                Data::Member(data) => match *data {
                    Some(Identifier { id, .. }) => Value::String(id),
                    None => Value::Null,
                },
                Data::Collection(data) => {
                    data.into_iter().map(|ident| ident.id).collect()
                }
            };

            map.insert(key, value);
        }

        Value::Object(map)
    }
}

impl Sealed for NewObject {}
