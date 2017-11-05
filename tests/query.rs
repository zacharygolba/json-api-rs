extern crate json_api;
#[macro_use]
extern crate ordermap;

use json_api::query::{self, Query};
use json_api::query::sort::Direction;
use ordermap::OrderMap;

fn from_mappings() -> OrderMap<&'static str, Query> {
    ordermap!{
        "" => Default::default(),
        "fields%5Barticles%5D=title" => Query::build()
            .fields("articles", vec!["title"])
            .finalize()
            .unwrap(),
        concat!(
            "fields%5Barticles%5D=body%2Ctitle%2Cpublished-at&",
            "fields%5Bcomments%5D=body&",
            "fields%5Busers%5D=name",
        ) => Query::build()
            .fields("articles", vec!["body", "title", "published-at"])
            .fields("comments", vec!["body"])
            .fields("users", vec!["name"])
            .finalize()
            .unwrap(),
        "filter%5Busers.name%5D=Alfred+Pennyworth" => Query::build()
            .filter("users.name", "Alfred Pennyworth")
            .finalize()
            .unwrap(),
        "include=author" => Query::build()
            .include("author")
            .finalize()
            .unwrap(),
        "include=author%2Ccomments%2Ccomments.author" => Query::build()
            .include("author")
            .include("comments")
            .include("comments.author")
            .finalize()
            .unwrap(),
        "page%5Bnumber%5D=0" => Query::build()
            .page(1, None)
            .finalize()
            .unwrap(),
        "page%5Bnumber%5D=1" => Query::build()
            .page(1, None)
            .finalize()
            .unwrap(),
        "page%5Bsize%5D=10" => Query::build()
            .page(1, Some(10))
            .finalize()
            .unwrap(),
        "page%5Bnumber%5D=2&page%5Bsize%5D=15" => Query::build()
            .page(2, Some(15))
            .finalize()
            .unwrap(),
        "sort=-published-at" => Query::build()
            .sort("published-at", Direction::Desc)
            .finalize()
            .unwrap(),
        "sort=published-at%2C-title" => Query::build()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .finalize()
            .unwrap(),
        "sort=published-at%2C-title%2C-author.name" => Query::build()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .sort("author.name", Direction::Desc)
            .finalize()
            .unwrap(),
        concat!(
            "fields%5Barticles%5D=body%2Ctitle%2Cpublished-at&",
            "fields%5Bcomments%5D=body&",
            "fields%5Busers%5D=name&",
            "filter%5Busers.name%5D=Alfred+Pennyworth&",
            "include=author%2Ccomments%2Ccomments.author&",
            "page%5Bnumber%5D=2&page%5Bsize%5D=15&",
            "sort=published-at%2C-title%2C-author.name",
        ) => Query::build()
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
            .finalize()
            .unwrap(),
    }
}

fn to_mappings() -> OrderMap<&'static str, Query> {
    from_mappings()
        .into_iter()
        .map(|(key, value)| match key {
            "page%5Bnumber%5D=0" => ("", value),
            "page%5Bnumber%5D=1" => ("", value),
            _ => (key, value),
        })
        .collect()
}

#[test]
fn query_from_slice() {
    for (source, expected) in from_mappings() {
        let actual = query::from_slice(source.as_bytes()).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_from_str() {
    for (source, expected) in from_mappings() {
        let actual = query::from_str(source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_string() {
    for (expected, source) in to_mappings() {
        let actual = query::to_string(&source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_vec() {
    for (expected, source) in to_mappings() {
        let actual = query::to_vec(&source).unwrap();
        assert_eq!(actual, expected.to_owned().into_bytes());
    }
}
