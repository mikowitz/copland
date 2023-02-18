use crate::container::Containable;
use crate::to_lilypond::{context_signature, delimiters, format_contents, ToLilypond};

#[derive(Debug)]
pub struct StaffGroup {
    contents: Vec<Containable>,
    name: Option<String>,
}

impl StaffGroup {
    #[must_use]
    pub fn new(contents: Vec<Containable>) -> Self {
        Self {
            contents,
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
}

impl ToLilypond for StaffGroup {
    fn to_lilypond(&self) -> Result<String, crate::error::Error>
    where
        Self: std::fmt::Debug,
    {
        // staff groups are always simultaneous
        let (open, close) = delimiters(true);
        Ok(format!(
            "{} {open}\n{}\n{close}",
            context_signature(&self.name, "StaffGroup"),
            format_contents(&self.contents)
        ))
    }
}
