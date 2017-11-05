# json-api

[![CircleCI branch](https://img.shields.io/circleci/project/github/zacharygolba/json-api-rs/master.svg?style=flat-square)](https://circleci.com/gh/zacharygolba/json-api-rs/tree/master) [![AppVeyor branch](https://img.shields.io/appveyor/ci/zacharygolba/json-api-rs/master.svg?logo=appveyor&style=flat-square)](https://ci.appveyor.com/project/zacharygolba/json-api-rs/branch/master) [![Codecov branch](https://img.shields.io/codecov/c/github/zacharygolba/json-api-rs/master.svg?style=flat-square)](https://codecov.io/gh/zacharygolba/json-api-rs) [![Crates.io](https://img.shields.io/crates/v/json-api.svg?style=flat-square)](https://crates.io/crates/json-api)

Idiomatic types for building a robust [JSON API](http://jsonapi.org/).

⚠️ **Warning**

> This crate is under active development. Some features have yet to be implemented,
> APIs are likely to change, and documentation is sparse.
>
> Consider this alpha quality software until this warning is removed.

## Features

### Serialization DSL

You can define a `Resource` using a friendly, declarative dsl.

#### Concise

```rust
pub struct Article {
    pub id: u64,
    pub body: String,
    pub title: String,
    pub author: User,
    pub comments: Vec<Comment>,
}

resource!(Article, |&self| {
    // Define the id with an expression that returns a string.
    id self.id.to_string();

    // Define the resource "type"
    kind "articles";

    // Define attributes with a comma seperated list of field names.
    attrs body, title;

    // Define relationships with a comma seperated list of field names.
    has_one author;
    has_many comments;
});
```

#### Flexible

```rust
pub struct Article {
    pub id: u64,
    pub body: String,
    pub title: String,
    pub author: User,
    pub comments: Vec<Comment>,
}

resource!(Article, |&self| {
    // Define the id with an expression that returns a string.
    id self.id.to_string();

    // Define the resource "type"
    kind "articles";

    // Define attributes with a comma seperated list of field names.
    attrs body, title;

    // Define a virtual attribute with a block expression
    attr "preview", {
        self.body
            .iter()
            .cloned()
            .take(140)
            .collect::<String>()
    }

    // Define a relationship with granular detail
    has_one "author", {
        // Data for has one should be Option<&T> where T: Resource
        data Some(&self.author);

        // Define relationship links
        link "self", format!("/articles/{}/relationships/author", self.id);
        link "related", format!("/articles/{}/author", self.id);

        // Define arbitrary meta members with a block expression
        meta "read-only", true
    }

    // Define a relationship with granular detail
    has_many "comments", {
        // Data for has one should be an Iterator<Item = &T> where T: Resource
        data self.comments.iter();

        // Define relationship links
        link "self", format!("/articles/{}/relationships/comments", self.id);
        link "related", format!("/articles/{}/comments", self.id);

        // Define arbitrary meta members with a block expression
        meta "total", {
            self.comments.len()
        }
    }

    // You can also define links with granular details as well
    link "self", {
        href format!("/articles/{}", self.id);
    }

    // Define arbitrary meta members with a block expression
    meta "copyright", {
        format!("© 2017 {}", self.author.full_name())
    }
});
```

### Rocket Support

The [json-api-rocket](https://crates.io/crates/json-api-rocket) crate provides responders
as well as a fairing for catching errors and returning [JSON API](http://jsonapi.org)
error documents.

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate json_api;
extern crate json_api_rocket;
extern crate rocket;

mod models;

use json_api_rocket::{Collection, ErrorHandler, Member};
use models::Article;

#[get("/")]
fn collection() -> Collection<Article> {
    (1..25).map(Article::new).collect()
}

#[get("/<id>")]
fn member(id: u64) -> Member<Article> {
    let article = Article::new(id);
    Member(article)
}

fn main() {
    rocket::ignite()
        .attach(ErrorHandler) // Adds JSON API error catchers
        .mount("/articles", routes![collection, member])
        .launch();
}

```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
