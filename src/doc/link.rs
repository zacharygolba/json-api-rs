use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use http::Uri;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use builder;
use error::Error;
use value::{Key, Map, Value};

#[derive(Clone, Default, PartialEq)]
pub struct Link {
    pub href: Uri,
    pub meta: Map<Key, Value>,
    /// Private field for backwards compatibility.
    _ext: (),
}

impl Link {
    pub fn build() -> LinkBuilder {
        Default::default()
    }
}

impl Debug for Link {
    fn fmt(&self, fmtr: &mut Formatter) -> fmt::Result {
        fmtr.debug_struct("Link")
            .field("href", &self.href)
            .field("meta", &self.meta)
            .finish()
    }
}

impl Deref for Link {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.href
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.href)
    }
}

impl FromStr for Link {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Link {
            href: value.parse()?,
            meta: Default::default(),
            _ext: (),
        })
    }
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Href,
            Meta,
        }

        struct LinkVisitor;

        impl<'de> Visitor<'de> for LinkVisitor {
            type Value = Link;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("string or a link object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Link, E>
            where
                E: de::Error,
            {
                value.parse().map_err(de::Error::custom)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Link, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut href = None;
                let mut meta = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Href if href.is_some() => {
                            return Err(de::Error::duplicate_field("href"))
                        }
                        Field::Meta if meta.is_some() => {
                            return Err(de::Error::duplicate_field("meta"))
                        }
                        Field::Href => {
                            let next = map.next_value::<String>()?;
                            href = Some(next.parse().map_err(de::Error::custom)?);
                        }
                        Field::Meta => {
                            meta = Some(map.next_value()?);
                        }
                    }
                }

                Ok(Link {
                    href: href.ok_or_else(|| de::Error::missing_field("href"))?,
                    meta: meta.unwrap_or_default(),
                    _ext: (),
                })
            }
        }

        deserializer.deserialize_any(LinkVisitor)
    }
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let href = self.href.to_string();
        let meta = &self.meta;

        if meta.is_empty() {
            serializer.serialize_str(&href)
        } else {
            let mut state = serializer.serialize_struct("Link", 2)?;

            state.serialize_field("href", &href)?;
            state.serialize_field("meta", meta)?;

            state.end()
        }
    }
}

#[derive(Debug, Default)]
pub struct LinkBuilder {
    href: Option<String>,
    meta: Vec<(String, Value)>,
}

impl LinkBuilder {
    pub fn finalize(&mut self) -> Result<Link, Error> {
        let href = builder::required("href", &mut self.href)?.parse()?;
        let meta = builder::map(&mut self.meta, Ok)?;

        Ok(Link {
            href,
            meta,
            _ext: (),
        })
    }

    pub fn href<V>(&mut self, value: V) -> &mut Self
    where
        V: AsRef<str>,
    {
        self.href = Some(value.as_ref().to_owned());
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
