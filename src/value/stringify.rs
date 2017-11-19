use std::borrow::Cow;

/// A specializable version of `std::string::ToString`.
pub trait Stringify {
    /// Returns the string representation of the give value as a byte vector.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Stringify;
    /// #
    /// # fn main() {
    /// use json_api::value::Stringify;
    /// assert_eq!(25.to_bytes(), vec![50, 53]);
    /// #
    /// # }
    /// ```
    fn to_bytes(&self) -> Vec<u8>;

    /// Returns the string representation of the given value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::value::Stringify;
    /// #
    /// # fn main() {
    /// use json_api::value::Stringify;
    /// assert_eq!(25.stringify(), "25");
    /// #
    /// # }
    /// ```
    fn stringify(&self) -> String {
        let bytes = self.to_bytes();
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

impl Stringify for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }

    fn stringify(&self) -> String {
        self.to_owned()
    }
}

impl<'a> Stringify for Cow<'a, str> {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }

    fn stringify(&self) -> String {
        self[..].to_owned()
    }
}

impl Stringify for str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }

    fn stringify(&self) -> String {
        self.to_owned()
    }
}

macro_rules! impl_stringify_for_display {
    { $($ty:ty),* $(,)* } => {
        $(impl Stringify for $ty {
            fn to_bytes(&self) -> Vec<u8> {
                self.to_string().into_bytes()
            }

            fn stringify(&self) -> String {
                self.to_string()
            }
        })*
    }
}

impl_stringify_for_display!{
    f32, f64,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
}
