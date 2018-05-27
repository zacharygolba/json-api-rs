use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

use error;

pub struct JsonApiFairing;

impl Fairing for JsonApiFairing {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Attach,
            name: "JsonApiFairing",
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let rocket = rocket.catch(error::catchers());
        Ok(rocket)
    }
}
