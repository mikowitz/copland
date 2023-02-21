use crate::container::Containable;
use crate::error::Error;
use crate::lilypond::{context_signature, delimiters, format_contents, ToLilypond};

#[derive(Debug)]
pub struct Score {
    contents: Vec<Containable>,
    name: Option<String>,
}

impl Score {
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

impl ToLilypond for Score {
    fn to_lilypond(&self) -> Result<String, Error> {
        // scores are always simultaneous
        let (open, close) = delimiters(true);
        Ok(format!(
            "{} {open}\n{}\n{close}",
            context_signature(&self.name, "Score"),
            format_contents(&self.contents)
        ))
    }
}

mod rhythmic_staff;
pub use rhythmic_staff::RhythmicStaff;
mod staff;
pub use staff::Staff;
mod staff_group;
pub use staff_group::StaffGroup;
mod voice;
pub use voice::Voice;

#[cfg(test)]
mod tests;
