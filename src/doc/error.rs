use doc::Link;
use http::StatusCode;
use value::{Key, Map};

/// Contains information about problems encountered while performing an
/// operation.
///
/// For more information, check out the *[error objects]* section of the JSON API
/// specification.
///
/// [error objects]: http://jsonapi.org/format/#error-objects
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ErrorObject {
    /// An application-specific error code, expressed as a string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// A human-readable explanation specific to this occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// A unique identifier for this particular occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Contains relevant links. If this value of this field is empty, it will not be
    /// serialized. For more information, check out the *[links]* section of the JSON
    /// API specification.
    ///
    /// [links]: https://goo.gl/E4E6Vt
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,

    /// Non-standard meta information. If this value of this field is empty, it will not
    /// be serialized. For more information, check out the *[meta information]* section
    /// of the JSON API specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,

    /// The source of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,

    /// The HTTP status code applicable to this problem.
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_status")]
    pub status: Option<StatusCode>,

    /// A short, human-readable summary of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl ErrorObject {
    /// Returns a new `ErrorObject` with the specified `status`.
    pub fn new(status: Option<StatusCode>) -> Self {
        let title = status
            .and_then(|value| value.canonical_reason())
            .map(|reason| reason.to_owned());

        ErrorObject {
            status,
            title,
            ..Default::default()
        }
    }
}

/// References to the source of the error.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorSource {
    /// A string indicating which query parameter caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,

    /// A JSON pointer to the associated entity in the request document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl ErrorSource {
    /// Returns a new `ErrorSource` with the specified `parameter` and
    /// `pointer` values.
    pub fn new(parameter: Option<String>, pointer: Option<String>) -> Self {
        ErrorSource {
            parameter,
            pointer,
            _ext: (),
        }
    }
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
