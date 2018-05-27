//! Idiomatic types for building a robust JSON API.

#[macro_use]
extern crate error_chain;
extern crate ordermap;
extern crate percent_encoding;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

pub extern crate http;

mod resource;

mod sealed {
    /// Private trait used to prevent marker traits from being implemented
    /// downstream.
    pub trait Sealed {}
}

pub mod doc;
pub mod error;
pub mod query;
pub mod value;
pub mod view;

#[doc(inline)]
pub use doc::Document;
#[doc(inline)]
pub use doc::{from_doc, from_reader, from_slice, from_str};
#[doc(inline)]
pub use doc::{
    to_doc, to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
};
#[doc(inline)]
pub use error::Error;
pub use resource::Resource;
#[doc(inline)]
pub use value::{from_value, to_value, Value};
