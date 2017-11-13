use serde::de::DeserializeOwned;
use std::ops::{Deref, DerefMut};

use json_api::{self, Error};
use json_api::doc::object::{NewObject, Object};
use json_api::query::{self, Page, Query as JsonApiQuery, Sort};
use json_api::value::{Key, Path, Value};
use json_api::value::collections::{map, set, Set};
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct Create<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> Create<T> {
    /// Consumes the `Create` wrapper and returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> Deref for Create<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> DerefMut for Create<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> FromData for Create<T> {
    type Error = Error;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        json_api::from_reader::<_, NewObject, _>(data.open())
            .map(Create)
            .into_outcome(Status::BadRequest)
    }
}

#[derive(Debug)]
pub struct Update<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> Update<T> {
    /// Consumes the `Update` wrapper and returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> Deref for Update<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> DerefMut for Update<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> FromData for Update<T> {
    type Error = Error;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        json_api::from_reader::<_, Object, _>(data.open())
            .map(Update)
            .into_outcome(Status::BadRequest)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    inner: JsonApiQuery,
}

impl Query {
    /// Consumes the [`Query`] wrapper and returns the wrapped value.
    ///
    /// [`Query`]: ./struct.Query.html
    pub fn into_inner(self) -> JsonApiQuery {
        self.inner
    }

    pub fn fields(&self) -> map::Iter<Key, Set> {
        self.inner.fields.iter()
    }

    pub fn filter(&self) -> map::Iter<Path, Value> {
        self.inner.filter.iter()
    }

    pub fn include(&self) -> set::Iter<Path> {
        self.inner.include.iter()
    }

    pub fn page(&self) -> Option<Page> {
        self.inner.page
    }

    pub fn sort(&self) -> set::Iter<Sort> {
        self.inner.sort.iter()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Query {
    type Error = Error;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let data = req.uri().query();

        data.map(query::from_str)
            .unwrap_or_else(|| Ok(Default::default()))
            .map(|inner| Query { inner })
            .into_outcome(Status::BadRequest)
    }
}
