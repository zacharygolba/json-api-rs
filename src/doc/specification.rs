use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::{Serialize, Serializer};

use builder;
use error::Error;
use value::{Key, Map, Value};

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct JsonApi {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map<Key, Value>,
    pub version: Version,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Debug for JsonApi {
    fn fmt(&self, fmtr: &mut Formatter) -> fmt::Result {
        fmtr.debug_struct("JsonApi")
            .field("meta", &self.meta)
            .field("version", &self.version)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct JsonApiBuilder {
    meta: Vec<(String, Value)>,
    version: Option<Version>,
}

impl JsonApiBuilder {
    pub fn finalize(&mut self) -> Result<JsonApi, Error> {
        let meta = builder::map(&mut self.meta, Ok)?;
        let version = self.version.unwrap_or_default();

        Ok(JsonApi {
            meta,
            version,
            _ext: (),
        })
    }

    pub fn meta<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: Into<Value>,
    {
        self.meta.push((key.as_ref().to_owned(), value.into()));
        self
    }

    pub fn version(&mut self, value: Version) -> &mut Self {
        self.version = Some(value);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Version {
    /// Version 1.0
    V1,
}

impl Version {
    fn as_str(&self) -> &str {
        match *self {
            Version::V1 => "1.0",
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::V1
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "1.0" => Ok(Version::V1),
            v => Err(Error::unsupported_version(v)),
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
