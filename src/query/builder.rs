use std::mem;

use error::Error;
use query::{Direction, Page, Query, Sort};
use value::{Key, Map, Path, Set, Value};

/// An implementation of the "builder pattern" that can be used to construct a
/// new query.
#[derive(Default)]
pub struct Builder {
    fields: Vec<(String, Vec<String>)>,
    filter: Vec<(String, Value)>,
    include: Vec<String>,
    page: Option<Page>,
    sort: Vec<(String, Direction)>,
}

impl Builder {
    /// Attempt to construct a new query from the previously supplied values.
    pub fn build(&mut self) -> Result<Query, Error> {
        Ok(Query {
            sort: {
                self.sort
                    .drain(..)
                    .map(|(field, direction)| {
                        let field = field.parse()?;
                        Ok(Sort::new(field, direction))
                    })
                    .collect::<Result<Set<Sort>, Error>>()?
            },
            filter: {
                self.filter
                    .drain(..)
                    .map(|(key, value)| Ok((key.parse()?, value)))
                    .collect::<Result<Map<Path, Value>, Error>>()?
            },
            fields: {
                self.fields
                    .drain(..)
                    .map(|(key, mut value)| {
                        let key = key.parse::<Key>()?;
                        let value = value
                            .drain(..)
                            .map(|item| item.parse())
                            .collect::<Result<Set, Error>>()?;

                        Ok((key, value))
                    })
                    .collect::<Result<Map<Key, Set>, Error>>()?
            },
            include: {
                self.include
                    .drain(..)
                    .map(|value| value.parse())
                    .collect::<Result<Set<Path>, Error>>()?
            },
            page: mem::replace(&mut self.page, None),
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
