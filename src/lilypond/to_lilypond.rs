use itertools::Itertools;

use crate::attachment::attachable::Components;
use crate::container::Containable;
use crate::error::Error;
use crate::prelude::Attachment;

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
    fn to_lilypond(&self) -> Result<String, Error>;
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

#[must_use]
pub fn indent(s: &str) -> String {
    s.lines().map(|s| format!("  {s}")).join("\n")
}

#[must_use]
pub fn prepare_attachments(attachments: &[Attachment]) -> (String, String) {
    let (before, after) = attachments
        .iter()
        .map(Attachment::prepared_components)
        .fold((vec![], vec![]), fold_attachment_tuples);

    (
        attachments_to_string(&before),
        attachments_to_string(&after),
    )
}

fn fold_attachment_tuples(acc: Components, el: Components) -> Components {
    let (mut before, mut after) = acc;
    let (new_before, new_after) = el;
    before.extend(new_before);
    after.extend(new_after);

    (before, after)
}

fn attachments_to_string(attachments: &[String]) -> String {
    attachments
        .iter()
        .fold(String::new(), |acc, el| format!("{acc}\n{el}"))
}
