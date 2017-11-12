pub mod page;
pub mod sort;

use percent_encoding::percent_decode;
use serde_qs;

use self::sort::Direction;
use builder;
use error::Error;
use value::{Key, Map, Path, Set, Value};

pub use self::page::Page;
pub use self::sort::Sort;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Query {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub fields: Map<Key, Set>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub filter: Map<Path, Value>,
    #[serde(default, skip_serializing_if = "Set::is_empty")]
    pub include: Set<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Page>,
    #[serde(default, skip_serializing_if = "Set::is_empty")]
    pub sort: Set<Sort>,
    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Query {
    pub fn builder() -> QueryBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct QueryBuilder {
    fields: Vec<(String, Vec<String>)>,
    filter: Vec<(String, Value)>,
    include: Vec<String>,
    page: Option<Page>,
    sort: Vec<(String, Direction)>,
}

impl QueryBuilder {
    pub fn build(&mut self) -> Result<Query, Error> {
        Ok(Query {
            fields: builder::iter(&mut self.fields, |(key, mut fields)| {
                let key = key.parse()?;
                let fields = fields
                    .drain(..)
                    .map(|item| item.parse())
                    .collect::<Result<_, _>>()?;

                Ok((key, fields))
            })?,
            filter: builder::iter(&mut self.filter, |(key, value)| Ok((key.parse()?, value)))?,
            include: builder::iter(&mut self.include, |key| key.parse())?,
            page: builder::optional(&mut self.page),
            sort: builder::iter(&mut self.sort, |(field, direction)| {
                Ok(Sort::new(field.parse()?, direction))
            })?,
            _ext: (),
        })
    }

    pub fn fields<I, K, V>(&mut self, key: K, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = iter.into_iter().map(|i| i.into()).collect();

        self.fields.push((key, value));
        self
    }

    pub fn filter<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        let key = key.into();
        let value = value.into();

        self.filter.push((key, value));
        self
    }

    pub fn include<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.include.push(value.into());
        self
    }

    pub fn page(&mut self, number: u64, size: Option<u64>) -> &mut Self {
        self.page = Some(Page::new(number, size));
        self
    }

    pub fn sort<F>(&mut self, field: F, direction: Direction) -> &mut Self
    where
        F: Into<String>,
    {
        self.sort.push((field.into(), direction));
        self
    }
}

pub fn from_slice(data: &[u8]) -> Result<Query, Error> {
    let value = percent_decode(data).decode_utf8()?;
    Ok(serde_qs::from_bytes(value.as_bytes())?)
}

pub fn from_str(data: &str) -> Result<Query, Error> {
    from_slice(data.as_bytes())
}

pub fn to_string(query: &Query) -> Result<String, Error> {
    use percent_encoding::{percent_encode, QUERY_ENCODE_SET};

    let value = serde_qs::to_string(query)?;
    let data = value.as_bytes();

    Ok(percent_encode(data, QUERY_ENCODE_SET).collect())
}

pub fn to_vec(query: &Query) -> Result<Vec<u8>, Error> {
    to_string(query).map(Vec::from)
}
