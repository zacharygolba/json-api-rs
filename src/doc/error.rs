use builder;
use doc::Link;
use http::StatusCode;
use value::{Key, Map, Value};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_status")]
    pub status: Option<StatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct ErrorBuilder {
    code: Option<String>,
    detail: Option<String>,
    id: Option<String>,
    links: Vec<(String, Link)>,
    meta: Vec<(String, Value)>,
    source: Option<Source>,
    status: Option<StatusCode>,
    title: Option<String>,
}

impl ErrorBuilder {
    pub fn build(&mut self) -> Result<Error, ::error::Error> {
        Ok(Error {
            code: builder::optional(&mut self.code),
            detail: builder::optional(&mut self.detail),
            id: builder::optional(&mut self.id),
            links: builder::iter(&mut self.links, builder::parse_key)?,
            meta: builder::iter(&mut self.meta, builder::parse_key)?,
            source: builder::optional(&mut self.source),
            status: builder::optional(&mut self.status),
            title: builder::optional(&mut self.title),
            _ext: (),
        })
    }

    pub fn code<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.code = Some(value.as_ref().to_owned());
        self
    }

    pub fn detail<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.detail = Some(value.as_ref().to_owned());
        self
    }

    pub fn id<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.id = Some(value.as_ref().to_owned());
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

    pub fn status(&mut self, value: StatusCode) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn title<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.title = Some(value.as_ref().to_owned());
        self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

mod serde_status {
    use std::fmt::{self, Formatter};

    use serde::de::{Deserializer, Error, Visitor};
    use serde::ser::Serializer;

    use http::StatusCode;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<StatusCode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusVisitor;

        impl<'de> Visitor<'de> for StatusVisitor {
            type Value = Option<StatusCode>;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("a string containing a http status code")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(self)
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map(Some).map_err(Error::custom)
            }
        }

        deserializer.deserialize_option(StatusVisitor)
    }

    pub fn serialize<S>(
        value: &Option<StatusCode>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *value {
            Some(status) => serializer.serialize_str(status.as_str()),
            None => serializer.serialize_none(),
        }
    }
}
