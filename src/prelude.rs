pub use actix::prelude::*;
pub use actix_web::*;
pub use diesel::prelude::*;
pub use failure::*;
pub use futures::prelude::*;

/// Local extensions for Result type
pub trait ResultExt<T, E> {
    /// Returns passed `err` wrapped to `Err()` if the result is `Err`
    ///
    /// Alias for `.map_err(|_| err)`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate howtocards;
    /// use howtocards::prelude::*;
    /// fn foo(x: u32) -> Result<u32, u32> { Err(x * 2) }
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum ExErr {
    ///   FooErr,
    /// }
    ///
    /// assert_eq!(foo(2).or_err(ExErr::FooErr).unwrap_err(), Err::<u32, _>(ExErr::FooErr).unwrap_err());
    /// assert_eq!(foo(2).or_err(ExErr::FooErr).unwrap_err(), foo(2).map_err(|_| ExErr::FooErr).unwrap_err());
    /// ```
    fn or_err(self, err: E) -> Result<T, E>;
}

impl<T, E, R> ResultExt<T, R> for Result<T, E> {
    fn or_err(self, err: R) -> Result<T, R> {
        self.map_err(|_| err)
    }
}
