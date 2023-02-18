use itertools::Itertools;

use crate::container::Containable;
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

#[must_use]
/// Formats a vector of container contents as Lilypond
///
/// # Panics
///
/// Will panic if any of the contents cannot be properly formatted as Lilypond
pub fn format_contents(contents: &[Containable]) -> String {
    indent(&contents.iter().map(|c| c.to_lilypond().unwrap()).join("\n"))
}

#[must_use]
pub fn delimiters(simultaneous: bool) -> (String, String) {
    if simultaneous {
        ("<<".to_string(), ">>".to_string())
    } else {
        ("{".to_string(), "}".to_string())
    }
}

#[must_use]
pub fn context_signature(name: &Option<String>, context_type: &str) -> String {
    name.as_ref().map_or_else(
        || format!("\\new {context_type}"),
        |name| format!("\\context {context_type} = \"{name}\""),
    )
}

fn indent(s: &str) -> String {
    s.lines().map(|s| format!("  {s}")).join("\n")
}
