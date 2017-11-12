//! Collection types with consistent ordering.

pub mod map;
pub mod set;

pub use ordermap::Equivalent;

pub use self::map::Map;
pub use self::set::Set;
