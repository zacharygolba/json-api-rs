use json_api::Error;
use json_api::query::{self, Page, Query as JsonApiQuery, Sort};
use json_api::value::{Set, Value};
use json_api::value::map::Iter as MapIter;
use json_api::value::set::Iter as SetIter;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

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

    pub fn fields(&self) -> MapIter<Set> {
        self.inner.fields.iter()
    }

    pub fn filter(&self) -> MapIter<Value> {
        self.inner.filter.iter()
    }

    pub fn include(&self) -> SetIter {
        self.inner.include.iter()
    }

    pub fn page(&self) -> Option<Page> {
        self.inner.page
    }

    pub fn sort(&self) -> Option<&Sort> {
        self.inner.sort.as_ref()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Query {
    type Error = Error;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let data = req.uri().query();
        let result = data.map(query::from_str)
            .unwrap_or_else(|| Ok(Default::default()))
            .map(|inner| Query { inner });

        match result {
            Ok(params) => Outcome::Success(params),
            Err(e) => Outcome::Failure((Status::BadRequest, e)),
        }
    }
}