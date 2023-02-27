use crate::container::Containable;
use crate::error::Error;
use crate::lilypond::{context_signature, delimiters, format_contents, ToLilypond};

#[derive(Debug, Clone)]
pub struct Staff {
    contents: Vec<Containable>,
    name: Option<String>,
    simultaneous: bool,
}

impl Staff {
    #[must_use]
    pub fn new(contents: Vec<Containable>) -> Self {
        Self {
            contents,
            simultaneous: false,
            name: None,
        }
    }

    #[must_use]
    pub fn contents(&self) -> &[Containable] {
        &self.contents
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    pub fn set_simultaneous(&mut self, simultaneous: bool) {
        self.simultaneous = simultaneous;
    }
}

impl ToLilypond for Staff {
    fn to_lilypond(&self) -> Result<String, Error> {
        let (open, close) = delimiters(self.simultaneous);
        Ok(format!(
            "{} {open}\n{}\n{close}",
            context_signature(&self.name, "Staff"),
            format_contents(&self.contents)
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn without_name() {
        let mut voice = Voice::new(vec![Rest::new(Duration::new(1, 4)).unwrap().into()]);
        voice.set_name("Voice One");
        voice.set_simultaneous(true);

        let staff = Staff::new(vec![voice.into()]);

        assert_eq!(
            staff.to_lilypond().unwrap(),
            r#"
\new Staff {
  \context Voice = "Voice One" <<
    r4
  >>
}
        "#
            .trim()
        )
    }

    #[test]
    fn simultaneous() {
        let mut voice = Voice::new(vec![Rest::new(Duration::new(1, 4)).unwrap().into()]);
        voice.set_name("Voice One");

        let mut voice2 = Voice::new(vec![Spacer::new(Duration::new(1, 4)).unwrap().into()]);
        voice2.set_name("Voice Two");

        let mut staff = Staff::new(vec![voice.into(), voice2.into()]);
        staff.set_name("Staff One");
        staff.set_simultaneous(true);

        assert_eq!(
            staff.to_lilypond().unwrap(),
            r#"
\context Staff = "Staff One" <<
  \context Voice = "Voice One" {
    r4
  }
  \context Voice = "Voice Two" {
    s4
  }
>>
        "#
            .trim()
        )
    }
}
