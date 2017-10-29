#[macro_use]
extern crate error_chain;
extern crate http;
extern crate ordermap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod builder;
mod resource;

mod sealed {
    /// Private trait used to prevent marker traits from being implemented downstream.
    pub trait Sealed {}
}

pub mod doc;
pub mod error;
pub mod value;

#[doc(inline)]
pub use doc::{Document, ErrorDocument};
pub use error::Error;
pub use resource::Resource;
#[doc(inline)]
pub use value::{from_value, to_value, Value};
