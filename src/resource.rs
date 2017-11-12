use doc::{Identifier, Object};
use error::Error;

pub trait Resource: Sized {
    fn ident(&self) -> Result<Identifier, Error>;
    fn object(&self) -> Result<Object, Error>;
}

#[macro_export]
macro_rules! resource {
    ($target:ident, |&$ctx:ident| { $($rest:tt)* }) => {
        impl $crate::Resource for $target {
            fn ident(&$ctx) -> Result<$crate::doc::Identifier, $crate::error::Error> {
                let mut ident = $crate::doc::Identifier::builder();

                expand_resource_impl!(@id $ctx, ident, {
                    $($rest)*
                });

                expand_resource_impl!(@kind $ctx, ident, {
                    $($rest)*
                });

                expand_resource_impl!(@meta $ctx, ident, {
                    $($rest)*
                });

                ident.build()
            }

            fn object(&$ctx) -> Result<$crate::doc::Object, $crate::error::Error> {
                let mut object = $crate::doc::Object::builder();

                expand_resource_impl!(@attrs $ctx, object, {
                    $($rest)*
                });

                expand_resource_impl!(@id $ctx, object, {
                    $($rest)*
                });

                expand_resource_impl!(@kind $ctx, object, {
                    $($rest)*
                });

                expand_resource_impl!(@links $ctx, object, {
                    $($rest)*
                });

                expand_resource_impl!(@meta $ctx, object, {
                    $($rest)*
                });

                expand_resource_impl!(@rel $ctx, object, {
                    $($rest)*
                });

                object.build()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_resource_impl {
    (@id $ctx:ident, $builder:ident, {
        id $value:block
        $($rest:tt)*
    }) => {
        $builder.id($value);
    };

    (@kind $ctx:ident, $builder:ident, {
        kind $value:block
        $($rest:tt)*
    }) => {
        $builder.kind($value);
    };

    (@attrs $ctx:ident, $builder:ident, {
        attr $key:expr, $value:block
        $($rest:tt)*
    }) => {
        $builder.attribute($key, $crate::to_value($value)?);
        expand_resource_impl!(@attrs $ctx, $builder, {
            $($rest)*
        });
    };

    (@attrs $ctx:ident, $builder:ident, {
        attr $field:ident;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@attrs $ctx, $builder, {
            attr stringify!($field), &$ctx.$field;
            $($rest)*
        });
    };

    (@attrs $ctx:ident, $builder:ident, {
        attrs $($field:ident),+;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@attrs $ctx, $builder,  {
            $(attr $field;)+
            $($rest)*
        });
    };

    (@rel $ctx:ident, $builder:ident, {
        has_many $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        $builder.relationship($key, {
            let mut relationship = $crate::doc::Relationship::builder();

            expand_resource_impl!(@has_many $ctx, relationship, {
                $($body)*
            });

            relationship.build()?
        });

        expand_resource_impl!(@rel $ctx, $builder, {
            $($rest)*
        });
    };

    (@rel $ctx:ident, $builder:ident, {
        has_one $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        $builder.relationship($key, {
            let mut relationship = $crate::doc::Relationship::builder();

            expand_resource_impl!(@has_one $ctx, relationship, {
                $($body)*
            });

            relationship.build()?
        });

        expand_resource_impl!(@rel $ctx, $builder, {
            $($rest)*
        });
    };

    (@rel $ctx:ident, $builder:ident, {
        has_many $($field:ident),*;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@rel $ctx, $builder, {
            $(has_many stringify!($field), { data $ctx.$field.iter(); })*
            $($rest)*
        });
    };

    (@rel $ctx:ident, $builder:ident, {
        has_one $($field:ident),*;
        $($rest:tt)*
    }) => {
        expand_resource_impl!(@rel $ctx, $builder, {
            $(has_one stringify!($field), { data $ctx.$field.as_ref(); })*
            $($rest)*
        });
    };

    (@has_many $ctx:ident, $builder:ident, {
        data $value:block
        $($rest:tt)*
    }) => {
        $builder.data({
            let value = ($value)
                .map($crate::Resource::ident)
                .collect::<Result<_, _>>()?;

            $crate::doc::Data::Collection(value)
        });

        expand_resource_impl!(@meta $ctx, $builder, {
            $($rest)*
        });

        expand_resource_impl!(@links $ctx, $builder, {
            $($rest)*
        });
    };

    (@has_one $ctx:ident, $builder:ident, {
        data $value:block
        $($rest:tt)*
    }) => {
        if let Some(value) = $value {
            let ident = $crate::Resource::ident(value)?;
            $builder.data($crate::doc::Data::Member(Box::new(Some(ident))));
        }

        expand_resource_impl!(@meta $ctx, $builder, {
            $($rest)*
        });

        expand_resource_impl!(@links $ctx, $builder, {
            $($rest)*
        });
    };

    (@links $ctx:ident, $builder:ident, {
        link $key:expr, { $($body:tt)* }
        $($rest:tt)*
    }) => {
        $builder.link($key, {
            expand_resource_impl!(@link $ctx, $builder, {
                $($body)*
            })
        });

        expand_resource_impl!(@links $ctx, $builder, {
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

    (@link $ctx:ident, $builder:ident, {
        href $value:block
        $($rest:tt)*
    }) => {{
        let mut link = $crate::doc::Link::builder();

        link.href($value);

        expand_resource_impl!(@meta $ctx, link, {
            $($rest)*
        });

        link.build()?
    }};

    (@meta $ctx:ident, $builder:ident, {
        meta $key:expr, $value:block
        $($rest:tt)*
    }) => {
        $builder.meta($key, $crate::to_value($value)?);
        expand_resource_impl!(@meta $ctx, $builder, {
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
