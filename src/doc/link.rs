use std::cmp::{Eq, PartialEq};
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;

use http::Uri;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use error::Error;
use value::Map;

/// A data structure containing a URL. Can be deserialized from either a string or link
/// object.
///
/// For more information, check out the *[links]* section of the JSON API specification.
///
/// # Example
///
/// ```
/// # extern crate json_api;
/// #
/// # use json_api::Error;
/// #
/// # fn example() -> Result<(), Error> {
/// use json_api::doc::Link;
/// "https://rust-lang.org".parse::<Link>()?;
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example().unwrap();
/// # }
/// ```
///
/// [links]: https://goo.gl/E4E6Vt
#[derive(Clone, Debug, Default)]
pub struct Link {
    /// The link’s URI.
    pub href: Uri,

    /// Non-standard meta information. If this value of this field is empty, the link
    /// will be serialized as a string containing the contents of `href`. For more
    /// information, check out the *[meta information]* section of the JSON API
    /// specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    pub meta: Map,

    /// Private field for backwards compatibility.
    _ext: (),
}

impl Deref for Link {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.href
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.href.fmt(f)
    }
}

impl Eq for Link {}

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

impl Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.href.hash(state)
    }
}

impl PartialEq for Link {
    fn eq(&self, rhs: &Link) -> bool {
        self.href == rhs.href
    }
}

impl PartialEq<Uri> for Link {
    fn eq(&self, rhs: &Uri) -> bool {
        self.href == *rhs
    }
}

impl<'a> PartialEq<&'a str> for Link {
    fn eq(&self, other: &&'a str) -> bool {
        self.href == *other
    }
}

impl<'a> PartialEq<Link> for &'a str {
    fn eq(&self, link: &Link) -> bool {
        *link == *self
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
