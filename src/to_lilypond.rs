use crate::error::Error;

pub trait ToLilypond {
    /// Returns an object formatted for Lilypond
    ///
    /// # Errors
    ///
    /// Will return an error if the object given cannot be formatted
    /// for Lilypond.
    ///
    /// ## Example
    ///
    /// An unprintable duration is still useful pre-rendering for calculations,
    /// but will return an error if it is used in a printed note or rest.
    fn to_lilypond(&self) -> Result<String, Error>
    where
        Self: std::fmt::Debug,
    {
        panic!("make sure you've implemented ToLilypond for {self:?}");
    }
}
