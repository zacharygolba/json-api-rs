use std::mem;

use doc::{Data, Document, Identifier, Object};
use error::Error;
use query::Query;
use value::Set;
use value::fields::Key;
use view::{Context, Render};

/// A trait indicating that the given type can be represented as a resource.
///
/// Implementing this trait manually is not recommended. The [`resource!`] macro provides
/// a friendly DSL that implements trait with some additional functionality.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate json_api;
///
/// struct Post(u64);
///
/// resource!(Post, |&self| {
///     kind "posts";
///     id self.0;
/// });
/// #
/// # fn main() {}
/// ```
///
/// [`resource!`]: ./macro.resource.html
pub trait Resource {
    /// Returns a key containing the type of resource.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate json_api;
    /// #
    /// # struct Post(u64);
    /// #
    /// # resource!(Post, |&self| {
    /// #     kind "posts";
    /// #     id self.0;
    /// # });
    /// #
    /// # fn main() {
    /// use json_api::Resource;
    ///
    /// let kind = Post::kind();
    /// assert_eq!(kind, "posts");
    /// # }
    /// ```
    fn kind() -> Key;

    /// Returns a given resource's id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate json_api;
    /// #
    /// # struct Post(u64);
    /// #
    /// # resource!(Post, |&self| {
    /// #     kind "posts";
    /// #     id self.0;
    /// # });
    /// #
    /// # fn main() {
    /// use json_api::Resource;
    ///
    /// let post = Post(25);
    /// assert_eq!(post.id(), "25");
    /// # }
    /// ```
    fn id(&self) -> String;

    /// Renders a given resource as an identifier object.
    ///
    ///
    /// Calling this function directly is not recommended. It is much more ergonomic to
    /// use the [`json_api::to_doc`] function.
    ///
    /// [`json_api::to_doc`]: ./fn.to_doc.html
    fn to_ident(&self, ctx: &mut Context) -> Result<Identifier, Error>;

    /// Renders a given resource as a resource object.
    ///
    /// Calling this function directly is not recommended. It is much more ergonomic to
    /// use the [`json_api::to_doc`] function.
    ///
    /// [`json_api::to_doc`]: ./fn.to_doc.html
    fn to_object(&self, ctx: &mut Context) -> Result<Object, Error>;
}

impl<'a, T: Resource> Render<Identifier> for &'a T {
    fn render(self, query: Option<&Query>) -> Result<Document<Identifier>, Error> {
        let mut incl = Set::new();
        let mut ctx = Context::new(T::kind(), query, &mut incl);

        self.to_ident(&mut ctx)?.render(query)
    }
}

impl<'a, T: Resource> Render<Identifier> for &'a [T] {
    fn render(self, query: Option<&Query>) -> Result<Document<Identifier>, Error> {
        let mut incl = Set::new();
        let mut ctx = Context::new(T::kind(), query, &mut incl);

        self.into_iter()
            .map(|item| item.to_ident(&mut ctx))
            .collect::<Result<Vec<_>, _>>()?
            .render(query)
    }
}

impl<'a, T: Resource> Render<Object> for &'a T {
    fn render(self, query: Option<&Query>) -> Result<Document<Object>, Error> {
        let mut incl = Set::new();
        let (data, links, meta) = {
            let mut ctx = Context::new(T::kind(), query, &mut incl);
            let mut obj = self.to_object(&mut ctx)?;
            let links = mem::replace(&mut obj.links, Default::default());
            let meta = mem::replace(&mut obj.meta, Default::default());

            (obj.into(), links, meta)
        };

        Ok(Document::Ok {
            data,
            links,
            meta,
            included: incl,
            jsonapi: Default::default(),
        })
    }
}

impl<'a, T: Resource> Render<Object> for &'a [T] {
    fn render(self, query: Option<&Query>) -> Result<Document<Object>, Error> {
        let mut incl = Set::new();
        let mut data = Vec::with_capacity(self.len());

        {
            let mut ctx = Context::new(T::kind(), query, &mut incl);

            for item in self {
                data.push(item.to_object(&mut ctx)?);
            }
        }

        Ok(Document::Ok {
            data: Data::Collection(data),
            links: Default::default(),
            meta: Default::default(),
            included: incl,
            jsonapi: Default::default(),
        })
    }
}

/// A DSL for implementing the `Resource` trait.
///
/// # Examples
///
/// The `resource!` macro is both concise and flexible. Many of the keywords are
/// overloaded to provide a higher level of customization when necessary.
///
/// Here is a simple example that simply defines the resources id, kind, attributes, and
/// relationships.
///
/// ```
/// #[macro_use]
/// extern crate json_api;
///
/// struct Post {
///     id: u64,
///     body: String,
///     title: String,
///     author: Option<User>,
///     comments: Vec<Comment>,
/// }
///
/// resource!(Post, |&self| {
///     // Define the id.
///     id self.id;
///
///     // Define the resource "type"
///     kind "posts";
///
///     // Define attributes with a comma seperated list of field names.
///     attrs body, title;
///
///     // Define relationships with a comma seperated list of field names.
///     has_one author;
///     has_many comments;
/// });
/// #
/// # struct User;
/// #
/// # resource!(User, |&self| {
/// #     kind "users";
/// #     id String::new();
/// # });
/// #
/// # struct Comment;
/// #
/// # resource!(Comment, |&self| {
/// #     kind "comments";
/// #     id String::new();
/// # });
/// #
/// # fn main() {}
/// ```
///
/// Now let's take a look at how we can use the same DSL to get a higher level
/// customization.
///
/// ```
/// #[macro_use]
/// extern crate json_api;
///
/// struct Post {
///     id: u64,
///     body: String,
///     title: String,
///     author: Option<User>,
///     comments: Vec<Comment>,
/// }
///
/// resource!(Post, |&self| {
///     kind "articles";
///     id self.id;
///
///     attrs body, title;
///
///     // Define a virtual attribute with an expression
///     attr "preview", {
///         self.body
///             .chars()
///             .take(140)
///             .collect::<String>()
///     }
///
///     // Define a relationship with granular detail
///     has_one "author", {
///         // Data for has one should be Option<&T> where T: Resource
///         data self.author.as_ref();
///
///         // Define relationship links
///         link "self", format!("/articles/{}/relationships/author", self.id);
///         link "related", format!("/articles/{}/author", self.id);
///
///         // Define arbitrary meta members with a block expression
///         meta "read-only", true
///     }
///
///     // Define a relationship with granular detail
///     has_many "comments", {
///         // Data for has one should be an Iterator<Item = &T> where T: Resource
///         data self.comments.iter();
///
///         // Define relationship links
///         link "self", format!("/articles/{}/relationships/comments", self.id);
///         link "related", format!("/articles/{}/comments", self.id);
///
///         // Define arbitrary meta members with a block expression
///         meta "total", {
///             self.comments.len()
///         }
///     }
///
///     // You can also define links with granular details as well
///     link "self", {
///         href format!("/articles/{}", self.id);
///     }
///
///     // Define arbitrary meta members an expression
///     meta "copyright", self.author.as_ref().map(|user| {
///         format!("© 2017 {}", user.full_name())
///     });
/// });
/// #
/// # struct User;
/// #
/// # impl User {
/// #     fn full_name(&self) -> String {
/// #         String::new()
/// #     }
/// # }
/// #
/// # resource!(User, |&self| {
/// #     kind "users";
/// #     id String::new();
/// # });
/// #
/// # struct Comment;
/// #
/// # resource!(Comment, |&self| {
/// #     kind "comments";
/// #     id String::new();
/// # });
/// #
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! resource {
    ($target:ident, |&$this:ident| { $($rest:tt)* }) => {
        impl $crate::Resource for $target {
            fn kind() -> $crate::value::Key {
                let raw = extract_resource_kind!({ $($rest)* }).to_owned();
                $crate::value::Key::from_raw(raw)
            }

            fn id(&$this) -> String {
                use $crate::value::Stringify as JsonApiStringifyTrait;
                extract_resource_id!({ $($rest)* }).stringify()
            }

            fn to_ident(
                &$this,
                _: &mut $crate::view::Context,
            ) -> Result<$crate::doc::Identifier, $crate::Error> {
                let mut ident = {
                    let kind = <$target as $crate::Resource>::kind();
                    let id = $crate::Resource::id($this);

                    $crate::doc::Identifier::new(kind, id)
                };

                {
                    let _meta = &mut ident.meta;
                    expand_resource_impl!(@meta $this, _meta, {
                        $($rest)*
                    });
                }

                Ok(ident)
            }

            fn to_object(
                &$this,
                ctx: &mut $crate::view::Context,
            ) -> Result<$crate::doc::Object, $crate::error::Error> {
                #[allow(dead_code)]
                fn item_kind<T: $crate::Resource>(_: &T) -> $crate::value::Key {
                    T::kind()
                }

                #[allow(dead_code)]
                fn iter_kind<'a, I, T>(_: &I) -> $crate::value::Key
                where
                    I: Iterator<Item = &'a T>,
                    T: $crate::Resource + 'a,
                {
                    T::kind()
                }

                let mut obj = {
                    let kind = <$target as $crate::Resource>::kind();
                    let id = $crate::Resource::id($this);

                    $crate::doc::Object::new(kind, id)
                };

                {
                    let _attrs = &mut obj.attributes;
                    expand_resource_impl!(@attrs $this, _attrs, ctx, {
                        $($rest)*
                    });
                }

                {
                    let _links = &mut obj.links;
                    expand_resource_impl!(@links $this, _links, {
                        $($rest)*
                    });
                }

                {
                    let _meta = &mut obj.meta;
                    expand_resource_impl!(@meta $this, _meta, {
                        $($rest)*
                    });
                }

                {
                    let _related = &mut obj.relationships;
                    expand_resource_impl!(@rel $this, _related, ctx, {
                        $($rest)*
                    });
                }

                Ok(obj)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_resource_impl {
    (@attrs $this:ident, $attrs:ident, $ctx:ident, {
        attr $key:expr, $value:block
        $($rest:tt)*
    }) => {
        if $ctx.field($key) {
            let key = $key.parse::<$crate::value::Key>()?;
            let value = $crate::to_value($value)?;

            $attrs.insert(key, value);
        }

        expand_resource_impl!(@attrs $this, $attrs, $ctx, {
            $($rest)*
        });
    };

    (@attrs $this:ident, $($arg:ident),*, { attr $field:ident; $($rest:tt)* }) => {
        expand_resource_impl!(@attrs $this, $($arg),*, {
            attr stringify!($field), &$this.$field;
            $($rest)*
        });
    };

    (@attrs $($arg:ident),*, { attrs $($field:ident),+; $($rest:tt)* }) => {
        expand_resource_impl!(@attrs $($arg),*, {
            $(attr $field;)+
            $($rest)*
        });
    };

    (@rel $this:ident, $related:ident, $ctx:ident, {
        has_many $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        if $ctx.field($key) {
            let key = $key.parse::<$crate::value::Key>()?;
            expand_resource_impl!(@has_many $this, $related, key, $ctx, {
                $($body)*
            });
        }

        expand_resource_impl!(@rel $this, $related, $ctx, {
            $($rest)*
        });
    };

    (@rel $this:ident, $related:ident, $ctx:ident, {
        has_one $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        if $ctx.field($key) {
            let key = $key.parse::<$crate::value::Key>()?;
            expand_resource_impl!(@has_one $this, $related, key, $ctx, {
                $($body)*
            });
        }

        expand_resource_impl!(@rel $this, $related, $ctx, {
            $($rest)*
        });
    };

    (@rel $this:ident, $($arg:ident),*, {
        has_many $($field:ident),*;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@rel $this, $($arg),*, {
            $(has_many stringify!($field), { data $this.$field.iter(); })*
            $($rest)*
        });
    };

    (@rel $this:ident, $($arg:ident),*, {
        has_one $($field:ident),*;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@rel $this, $($arg),*, {
            $(has_one stringify!($field), { data $this.$field.as_ref(); })*
            $($rest)*
        });
    };

    (@has_many $this:ident, $related:ident, $key:ident, $ctx:ident, {
        data $value:block
        $($rest:tt)*
    }) => {
        let mut rel = $crate::doc::Relationship::new({
            let mut ctx = $ctx.fork(iter_kind(&$value), &$key);
            let mut data = match $value.size_hint() {
                (_, Some(size)) => Vec::with_capacity(size),
                _ => Vec::new(),
            };

            if ctx.included() {
                for item in $value {
                    let object = $crate::Resource::to_object(item, &mut ctx)?;
                    let ident = $crate::doc::Identifier::from(&object);

                    ctx.include(object);
                    data.push(ident);
                }
            } else {
                for item in $value {
                    data.push($crate::Resource::to_ident(item, &mut ctx)?);
                }
            }

            data.into()
        });

        {
            let links = &mut rel.links;
            expand_resource_impl!(@links $this, links, {
                $($rest)*
            });
        }

        {
            let _meta = &mut rel.meta;
            expand_resource_impl!(@meta $this, _meta, {
                $($rest)*
            });
        }

        $related.insert($key, rel);
    };

    (@has_one $this:ident, $related:ident, $key:ident, $ctx:ident, {
        data $value:block
        $($rest:tt)*
    }) => {
        let mut rel = $crate::doc::Relationship::new({
            let mut data = None;

            if let Some(item) = $value {
                let mut ctx = $ctx.fork(item_kind(item), &$key);

                data = Some($crate::Resource::to_ident(item, &mut ctx)?);

                if ctx.included() {
                    let object = $crate::Resource::to_object(item, &mut ctx)?;
                    ctx.include(object);
                }
            }

            data.into()
        });

        {
            let _links = &mut rel.links;
            expand_resource_impl!(@links $this, _links, {
                $($rest)*
            });
        }

        {
            let _meta = &mut rel.meta;
            expand_resource_impl!(@meta $this, _meta, {
                $($rest)*
            });
        }

        $related.insert($key, rel);
    };

    (@links $this:ident, $links:ident, {
        link $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        {
            let key = $key.parse::<$crate::value::Key>()?;
            let link = expand_resource_impl!(@link $this, {
                $($body)*
            });

            $links.insert(key, link);
        }

        expand_resource_impl!(@links $this, $links, {
            $($rest)*
        });
    };

    (@links $($args:ident),+, {
        link $key:expr, $value:expr;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@links $($args),+, {
            link $key, { href { $value } }
            $($rest)*
        });
    };

    (@link $this:ident, { href $value:block $($rest:tt)* }) => {{
        let mut link = $value.parse::<$crate::doc::Link>()?;

        {
            let _meta = &link.meta;
            expand_resource_impl!(@meta $this, _meta, {
                $($rest)*
            });
        }

        link
    }};

    (@meta $this:ident, $meta:ident, {
        meta $key:expr, $value:block
        $($rest:tt)*
    }) => {
        {
            let key = $key.parse::<$crate::value::Key>()?;
            let value = $crate::to_value($value)?;

            $meta.insert(key, value);
        }

        expand_resource_impl!(@meta $this, $meta, {
            $($rest)*
        });
    };

    // Ignore has_many specific syntax in other scopes.
    (@$scope:tt $($args:ident),+, {
        has_many $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    // Ignore has_one specific syntax in other scopes.
    (@$scope:tt $($args:ident),+, {
        has_one $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    // Ignore link specific syntax in other scopes.
    (@$scope:tt $($args:ident),+, {
        link $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        $kwd:ident $value:expr;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $kwd { $value }
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        has_many $key:expr, $value:block
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        has_one $key:expr, $value:block
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        link $key:expr, $value:block
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        $kwd:ident $key:expr, $value:expr;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $kwd $key, { $value }
            $($rest)*
        });
    };

    (@$scope:tt $($args:ident),+, {
        $skip:tt
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@$scope $($args),+, {
            $($rest)*
        });
    };

    ($($rest:tt)*) => ();
}

#[doc(hidden)]
#[macro_export]
macro_rules! extract_resource_id {
    ({ id $value:block $($rest:tt)* }) => { $value };
    ({ id $value:expr; $($rest:tt)* }) => { $value };
    ({ $skip:tt $($rest:tt)* }) => { extract_resource_id!({ $($rest)* }) };
    ({ $($rest:tt)* }) => ();
}

#[doc(hidden)]
#[macro_export]
macro_rules! extract_resource_kind {
    ({ kind $value:block $($rest:tt)* }) => { $value };
    ({ kind $value:expr; $($rest:tt)* }) => { $value };
    ({ $skip:tt $($rest:tt)* }) => { extract_resource_kind!({ $($rest)* }) };
    ({ $($rest:tt)* }) => ();
}
