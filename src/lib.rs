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

mod builder;
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

#[doc(inline)]
pub use doc::{Document, ErrorDocument};
pub use error::Error;
pub use resource::Resource;
#[doc(inline)]
pub use value::{from_value, to_value, Value};
