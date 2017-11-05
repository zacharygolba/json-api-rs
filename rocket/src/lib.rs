extern crate json_api;
extern crate rocket;
extern crate serde;
extern crate serde_json;

mod error;
mod fairing;

pub mod request;
pub mod response;

pub use self::fairing::JsonApiFairing;
pub use self::request::*;
pub use self::response::*;
