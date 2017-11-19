use doc::{Data, Document, PrimaryData};
use error::Error;
use query::Query;

/// A trait to render a given type as a document.
///
/// This trait is automatically implemented for any type which implements [`Resource`].
///
/// [`Resource`]: ../trait.Resource.html
pub trait Render<T: PrimaryData> {
    /// Attempts to render the given type as a document.
    ///
    /// Types that implement the [`Resource`] trait via the [`resource!`] macro can use
    /// the optional query argument to match object field-sets and included resources
    /// with what is present in the query.
    ///
    /// If a query does not have a matching field-set for a given type and the type in
    /// question is a part of the document's primary data or included resources, each
    /// attribute specified in the type's [`resource!`] macro invocation will be used.
    ///
    /// [`Resource`]: ../trait.Resource.html
    /// [`resource!`]: ../macro.resource.html
    fn render(self, query: Option<&Query>) -> Result<Document<T>, Error>;
}

impl<D, T> Render<D> for Option<T>
where
    D: PrimaryData,
    T: Render<D> + Sized,
{
    fn render(self, query: Option<&Query>) -> Result<Document<D>, Error> {
        match self {
            Some(value) => value.render(query),
            None => Ok(Document::Ok {
                data: Data::Member(Box::new(None)),
                included: Default::default(),
                jsonapi: Default::default(),
                links: Default::default(),
                meta: Default::default(),
            }),
        }
    }
}
