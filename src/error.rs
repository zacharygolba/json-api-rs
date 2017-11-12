use std::fmt::Display;
use std::str::Utf8Error;

use http::status::InvalidStatusCode as InvalidStatusCodeError;
use http::uri::InvalidUri as InvalidUriError;
use serde::de;
use serde_json::Error as JsonError;
use serde_qs::Error as QueryError;

error_chain!{
    foreign_links {
        InvalidStatusCode(InvalidStatusCodeError);
        InvalidUri(InvalidUriError);
        Json(JsonError);
        Query(QueryError);
        Utf8(Utf8Error);
    }

    errors {
        InvalidMemberName(name: String) {
            description("TODO")
            display("TODO")
        }

        MissingField(name: String) {
            description("A struct was built without a required field.")
            display(r#"missing required field "{}""#, name)
        }

        UnsupportedVersion(version: String) {
            description("The specified version of is not \
                         supported by this implementation.")
            display(r#"Version "{}" is not yet supported by \
                       this implementation."#, version)
        }
    }
}

impl Error {
    pub fn missing_field(name: &str) -> Self {
        Self::from(ErrorKind::MissingField(name.to_owned()))
    }

    pub fn unsupported_version(version: &str) -> Self {
        Self::from(ErrorKind::UnsupportedVersion(version.to_owned()))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::from(ErrorKind::Msg(format!("{}", msg)))
    }
}
