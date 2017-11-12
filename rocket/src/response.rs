use std::io::Cursor;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use json_api::{Document, Resource};
use json_api::doc::Data;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use serde::Serialize;

#[derive(Debug)]
pub struct Collection<T: Resource>(pub Vec<T>);

impl<T: Resource> Collection<T> {
    /// Consumes the [`Collection`] wrapper and returns the wrapped value.
    ///
    /// [`Collection`]: ./struct.Collection.html
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: Resource> Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Resource> FromIterator<T> for Collection<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Collection(Vec::from_iter(iter))
    }
}

impl<T: Resource> Responder<'static> for Collection<T> {
    fn respond_to(self, _request: &Request) -> Result<Response<'static>, Status> {
        self.into_iter()
            .map(Resource::object)
            .collect::<Result<Vec<_>, _>>()
            .map(Data::Collection)
            .and_then(|data| Document::builder().data(data).build())
            .map_err(|_| Status::InternalServerError)
            .and_then(with_body)
            .map(|mut resp| {
                resp.set_status(Status::Ok);
                resp
            })
    }
}

#[derive(Debug)]
pub struct Created<T: Resource>(pub T);

impl<T: Resource> Created<T> {
    /// Consumes the [`Created`] wrapper and returns the wrapped value.
    ///
    /// [`Created`]: ./struct.Created.html
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Resource> Deref for Created<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Created<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Resource> Responder<'static> for Created<T> {
    fn respond_to(self, _request: &Request) -> Result<Response<'static>, Status> {
        self.object()
            .and_then(|obj| Document::builder().data(obj).build())
            .map_err(|_| Status::InternalServerError)
            .and_then(with_body)
            .map(|mut resp| {
                resp.set_status(Status::Created);
                resp
            })
    }
}

#[derive(Debug)]
pub struct Member<T>(pub T);

impl<T: Resource> Member<T> {
    /// Consumes the [`Member`] wrapper and returns the wrapped value.
    ///
    /// [`Member`]: ./struct.Member.html
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Resource> Deref for Member<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Member<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Resource> Responder<'static> for Member<T> {
    fn respond_to(self, _request: &Request) -> Result<Response<'static>, Status> {
        self.object()
            .and_then(|obj| Document::builder().data(obj).build())
            .map_err(|_| Status::InternalServerError)
            .and_then(with_body)
            .map(|mut resp| {
                resp.set_status(Status::Ok);
                resp
            })
    }
}

pub(crate) fn with_body<T>(value: T) -> Result<Response<'static>, Status>
where
    T: Serialize,
{
    ::serde_json::to_vec(&value)
        .map_err(|_| Status::InternalServerError)
        .map(|body| {
            Response::build()
                .raw_header("Content-Type", "application/vnd.api+json")
                .sized_body(Cursor::new(body))
                .finalize()
        })
}
