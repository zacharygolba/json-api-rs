use std::fmt::{self, Debug, Formatter};

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Page {
    pub number: u64,
    pub size: Option<u64>,
    /// Private field for backwards compatibility.
    _ext: (),
}

impl Page {
    pub fn new(number: u64, size: Option<u64>) -> Self {
        let number = if number > 0 { number } else { 1 };

        Page {
            number,
            size,
            _ext: (),
        }
    }
}

impl Debug for Page {
    fn fmt(&self, fmtr: &mut Formatter) -> fmt::Result {
        fmtr.debug_struct("Page")
            .field("number", &self.number)
            .field("size", &self.size)
            .finish()
    }
}

impl<'de> Deserialize<'de> for Page {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};

        const FIELDS: &'static [&'static str] = &["number", "size"];

        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Number,
            Size,
        }

        struct PageVisitor;

        impl<'de> Visitor<'de> for PageVisitor {
            type Value = Page;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an object containing json api pagination parameters")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut number = None;
                let mut size = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        Field::Number if number.is_some() => {
                            return Err(Error::duplicate_field("number"));
                        }
                        Field::Size if size.is_some() => {
                            return Err(Error::duplicate_field("size"));
                        }
                        Field::Number => {
                            number = access.next_value()?;
                        }
                        Field::Size => {
                            size = access.next_value()?;
                        }
                    }
                }

                Ok(Page::new(number.unwrap_or(1), size))
            }
        }

        deserializer.deserialize_struct("Page", FIELDS, PageVisitor)
    }
}

impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Page", 2)?;
        let number = &self.number;
        let size = &self.size;

        if *number != 1 {
            state.serialize_field("number", number)?;
        }

        if let Some(ref value) = *size {
            state.serialize_field("size", value)?;
        }

        state.end()
    }
}
