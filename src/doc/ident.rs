use builder;
use error::Error;
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
