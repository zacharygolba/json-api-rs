use doc::Object;
use query::Query;
use value::fields::{Key, Path, Segment};
use value::Set;

/// A data structure containing render context that can be "forked" and passed
/// to a child context.
///
/// This struct is helpful if you want recursively call [`Resource::to_object`] to render
/// a document's primary data and included resources.
///
/// Since the `Context` struct requires a mutable (unique) reference to a document's
/// included resources, only one context can be operated on at a time. In other words, if
/// you want to access a context, it cannot have any children in scope. Since you can
/// only operate on a single context at time, a recursive implementation of [included
/// resources] and [sparse field-sets] is much easier.
///
/// [`Resource::to_object`]: ../trait.Resource.html#tymethod.to_object
/// [included resources]: http://jsonapi.org/format/#fetching-includes
/// [sparse field-sets]: http://jsonapi.org/format/#fetching-sparse-fieldsets
#[derive(Debug)]
pub struct Context<'v> {
    incl: &'v mut Set<Object>,
    kind: Key,
    path: Path,
    query: Option<&'v Query>,
}

impl<'v> Context<'v> {
    /// Creates a new, root context.
    ///
    /// This constructor can only be used when creating a root context. A child context
    /// can be created with the `fork` method.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// use json_api::value::Set;
    /// use json_api::view::Context;
    ///
    /// let mut included = Set::new();
    /// let mut ctx = Context::new("posts".parse()?, None, &mut included);
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// # example().unwrap();
    /// # }
    /// ```
    pub fn new(kind: Key, query: Option<&'v Query>, included: &'v mut Set<Object>) -> Self {
        Context {
            kind,
            query,
            incl: included,
            path: Path::new(),
        }
    }

    /// Returns true if the field name is present in the current context's
    /// field-set or the current context's field-set does not exist.
    pub fn field(&self, name: &str) -> bool {
        self.query
            .and_then(|q| q.fields.get(&self.kind))
            .map_or(true, |f| f.contains(name))
    }

    /// Creates a new child context from `self`.
    pub fn fork(&mut self, kind: Key, key: &Key) -> Context {
        Context {
            kind,
            incl: self.incl,
            path: self.path.join(key),
            query: self.query,
        }
    }

    /// Adds the `value` to the context's included resource set.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned.
    pub fn include(&mut self, value: Object) -> bool {
        self.incl.insert(value)
    }

    /// Returns `true` if the context is valid with respect to parent context(s).
    ///
    /// If there is no parent context (i.e the current context represents the primary
    /// data of the document), this will always return `false`.
    ///
    /// if there is a parent context and this function returns `false`, this context can
    /// should be ignored.
    pub fn included(&self) -> bool {
        self.query.map_or(false, |q| q.include.contains(&self.path))
    }
}
