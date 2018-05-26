extern crate json_api;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate serde;
extern crate serde_json;

mod error;
mod fairing;

mod config {
    use std::env;

    use rocket::config::Environment;

    lazy_static! {
        pub static ref ROCKET_ENV: Environment = {
            match env::var("ROCKET_ENV").ok() {
                Some(value) => value.parse().unwrap_or(Environment::Development),
                None => Environment::Development,
            }
        };
    }
}

pub mod request;
pub mod response;

pub use self::fairing::JsonApiFairing;
pub use self::request::*;
pub use self::response::*;
