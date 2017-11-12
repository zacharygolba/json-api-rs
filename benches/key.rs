#![feature(test)]

extern crate json_api;
extern crate test;

use std::str::FromStr;

use json_api::value::Key;
use test::Bencher;

const SOURCES: [&str; 6] = [
    "articles",
    "comments",
    "likes",
    "notification_settings",
    "shoppingCarts",
    "users",
];

#[bench]
fn from_str(b: &mut Bencher) {
    b.iter(|| for source in &SOURCES {
        Key::from_str(source).unwrap();
    })
}
