use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde_json;

use doc::{Data, Document, PrimaryData};
use error::Error;
use value::{self, Value};

pub trait IntoDocument<T: PrimaryData> {
    #[doc(hidden)]
    fn to_doc(self) -> Result<Document<T>, Error>;
}

impl<D, T> IntoDocument<D> for Option<T>
where
    D: PrimaryData,
    T: IntoDocument<D>,
{
    fn to_doc(self) -> Result<Document<D>, Error> {
        match self {
            Some(value) => value.to_doc(),
            None => Ok(Document::new(Data::Member(Box::new(None)))),
        }
    }
}

pub fn from_doc<T, U>(doc: Document<T>) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    let Document { data, included, .. } = doc;
    let value = value::convert::to_json(match data {
        Data::Member(data) => match *data {
            Some(item) => item.flatten(&included),
            None => Value::Null,
        },
        Data::Collection(data) => data.into_iter()
            .map(|item| item.flatten(&included))
            .collect(),
    });

    Ok(serde_json::from_value(value)?)
}

pub fn from_reader<R, T, U>(data: R) -> Result<U, Error>
where
    R: Read,
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_reader(data)?)
}

pub fn from_slice<T, U>(data: &[u8]) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_slice(data)?)
}

pub fn from_str<T, U>(data: &str) -> Result<U, Error>
where
    T: PrimaryData,
    U: DeserializeOwned,
{
    from_doc::<T, _>(serde_json::from_str(data)?)
}

pub fn to_doc<T, U>(value: T) -> Result<Document<U>, Error>
where
    T: IntoDocument<U>,
    U: PrimaryData,
{
    value.to_doc()
}

pub fn to_string<T, U>(value: T) -> Result<String, Error>
where
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_string(&to_doc(value)?)?)
}

pub fn to_string_pretty<T, U>(value: T) -> Result<String, Error>
where
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_string_pretty(&to_doc(value)?)?)
}

pub fn to_vec<T, U>(value: T) -> Result<Vec<u8>, Error>
where
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_vec(&to_doc(value)?)?)
}

pub fn to_vec_pretty<T, U>(value: T) -> Result<Vec<u8>, Error>
where
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_vec_pretty(&to_doc(value)?)?)
}

pub fn to_writer<W, T, U>(writer: W, value: T) -> Result<(), Error>
where
    W: Write,
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_writer(writer, &to_doc(value)?)?)
}

pub fn to_writer_pretty<W, T, U>(writer: W, value: T) -> Result<(), Error>
where
    W: Write,
    T: IntoDocument<U>,
    U: PrimaryData,
{
    Ok(serde_json::to_writer_pretty(writer, &to_doc(value)?)?)
}
