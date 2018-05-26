use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde_json;

use doc::{Data, Document, PrimaryData};
use error::Error;
use query::Query;
use value::{self, Value};
use view::Render;

/// Interpret a `Document<T>` as a type `U`.
pub fn from_doc<T, U>(doc: Document<T>) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    match doc {
        Document::Ok { data, included, .. } => {
            let value = value::convert::to_json(match data {
                Data::Member(data) => match *data {
                    Some(item) => item.flatten(&included),
                    None => Value::Null,
                },
                Data::Collection(data) => data
                    .into_iter()
                    .map(|item| item.flatten(&included))
                    .collect(),
            });

            Ok(serde_json::from_value(value)?)
        }
        Document::Err { .. } => {
            let e = Error::from("Document contains one or more error(s)");
            Err(e)
        }
    }
}

/// Deserialize a `Document<T>` from an IO stream of JSON text and then
/// iterpret it as a type `U`.
pub fn from_reader<R, T, U>(data: R) -> Result<U, Error>
where
    R: Read,
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_reader(data)?)
}

/// Deserialize a `Document<T>` from bytes of JSON text and then iterpret it as
/// a type `U`.
pub fn from_slice<T, U>(data: &[u8]) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_slice(data)?)
}

/// Deserialize a `Document<T>` from a string of JSON text and then iterpret it
/// as a type `U`.
pub fn from_str<T, U>(data: &str) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_str(data)?)
}

/// Render type `T` as a `Document<U>`.
pub fn to_doc<T, U>(value: T, query: Option<&Query>) -> Result<Document<U>, Error>
where
    T: Render<U>,
    U: PrimaryData,
{
    value.render(query)
}

/// Render type `T` as a `Document<U>` and then serialize it as a string of
/// JSON.
pub fn to_string<T, U>(value: T, query: Option<&Query>) -> Result<String, Error>
where
    T: Render<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_string(&to_doc(value, query)?)?)
}

/// Render type `T` as a `Document<U>` and then serialize it as a
/// pretty-printed string of JSON.
pub fn to_string_pretty<T, U>(value: T, query: Option<&Query>) -> Result<String, Error>
where
    T: Render<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_string_pretty(&to_doc(value, query)?)?)
}

/// Render type `T` as a `Document<U>` and then serialize it as a JSON byte
/// vector.
pub fn to_vec<T, U>(value: T, query: Option<&Query>) -> Result<Vec<u8>, Error>
where
    T: Render<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_vec(&to_doc(value, query)?)?)
}

/// Render type `T` as a `Document<U>` and then serialize it as a
/// pretty-printed JSON byte vector.
pub fn to_vec_pretty<T, U>(value: T, query: Option<&Query>) -> Result<Vec<u8>, Error>
where
    T: Render<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_vec_pretty(&to_doc(value, query)?)?)
}

/// Render type `T` as a `Document<U>` and then serialize it as JSON into the
/// IO stream.
pub fn to_writer<W, T, U>(writer: W, value: T, query: Option<&Query>) -> Result<(), Error>
where
    W: Write,
    T: Render<U>,
    U: PrimaryData,
{
    serde_json::to_writer(writer, &to_doc(value, query)?)?;
    Ok(())
}

/// Render type `T` as a `Document<U>` and then serialize it as pretty-printed
/// JSON into the IO stream.
pub fn to_writer_pretty<W, T, U>(writer: W, value: T, query: Option<&Query>) -> Result<(), Error>
where
    W: Write,
    T: Render<U>,
    U: PrimaryData,
{
    serde_json::to_writer_pretty(writer, &to_doc(value, query)?)?;
    Ok(())
}
