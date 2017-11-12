extern crate json_api;
#[macro_use]
extern crate ordermap;

use json_api::Error;
use json_api::query::{self, Query};
use json_api::query::sort::Direction;
use ordermap::OrderMap;

type Mapping = OrderMap<&'static str, Query>;

fn from_mapping() -> Result<Mapping, Error> {
    Ok(ordermap!{
        "" => Default::default(),
        "fields%5Barticles%5D=title" => Query::builder()
            .fields("articles", vec!["title"])
            .build()?,
        concat!(
            "fields%5Barticles%5D=body%2Ctitle%2Cpublished-at&",
            "fields%5Bcomments%5D=body&",
            "fields%5Busers%5D=name",
        ) => Query::builder()
            .fields("articles", vec!["body", "title", "published-at"])
            .fields("comments", vec!["body"])
            .fields("users", vec!["name"])
            .build()?,
        "filter%5Busers.name%5D=Alfred+Pennyworth" => Query::builder()
            .filter("users.name", "Alfred Pennyworth")
            .build()?,
        "include=author" => Query::builder()
            .include("author")
            .build()?,
        "include=author%2Ccomments%2Ccomments.author" => Query::builder()
            .include("author")
            .include("comments")
            .include("comments.author")
            .build()?,
        "page%5Bnumber%5D=0" => Query::builder()
            .page(1, None)
            .build()?,
        "page%5Bnumber%5D=1" => Query::builder()
            .page(1, None)
            .build()?,
        "page%5Bsize%5D=10" => Query::builder()
            .page(1, Some(10))
            .build()?,
        "page%5Bnumber%5D=2&page%5Bsize%5D=15" => Query::builder()
            .page(2, Some(15))
            .build()?,
        "sort=-published-at" => Query::builder()
            .sort("published-at", Direction::Desc)
            .build()?,
        "sort=published-at%2C-title" => Query::builder()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .build()?,
        "sort=published-at%2C-title%2C-author.name" => Query::builder()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .sort("author.name", Direction::Desc)
            .build()?,
        concat!(
            "fields%5Barticles%5D=body%2Ctitle%2Cpublished-at&",
            "fields%5Bcomments%5D=body&",
            "fields%5Busers%5D=name&",
            "filter%5Busers.name%5D=Alfred+Pennyworth&",
            "include=author%2Ccomments%2Ccomments.author&",
            "page%5Bnumber%5D=2&page%5Bsize%5D=15&",
            "sort=published-at%2C-title%2C-author.name",
        ) => Query::builder()
            .fields("articles", vec!["body", "title", "published-at"])
            .fields("comments", vec!["body"])
            .fields("users", vec!["name"])
            .filter("users.name", "Alfred Pennyworth")
            .include("author")
            .include("comments")
            .include("comments.author")
            .page(2, Some(15))
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .sort("author.name", Direction::Desc)
            .build()?,
    })
}

fn to_mapping() -> Result<Mapping, Error> {
    let mapping = from_mapping()?
        .into_iter()
        .map(|(key, value)| match key {
            "page%5Bnumber%5D=0" | "page%5Bnumber%5D=1" => ("", value),
            _ => (key, value),
        })
        .collect();

    Ok(mapping)
}

#[test]
fn query_from_slice() {
    for (source, expected) in from_mapping().unwrap() {
        let actual = query::from_slice(source.as_bytes()).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_from_str() {
    for (source, expected) in from_mapping().unwrap() {
        let actual = query::from_str(source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_string() {
    for (expected, source) in to_mapping().unwrap() {
        let actual = query::to_string(&source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_vec() {
    for (expected, source) in to_mapping().unwrap() {
        let actual = query::to_vec(&source).unwrap();
        assert_eq!(actual, expected.to_owned().into_bytes());
    }
}
