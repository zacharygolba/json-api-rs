//! Low-level utilies for generically rendering a document.
//!
//! The public API in this module is not the most ergonomic way to create a document from
//! a given type. The API in this module is exposed to provide way for library authors to
//! add custom rendering logic for types type they wish to implement in their crate. If
//! your looking for a simple way to render data as a document, check out the [functions
//! exported from the crate root].
//!
//! [functions exported from the crate root]: ../index.html#functions

mod context;
mod render;

pub use self::context::Context;
pub use self::render::Render;
