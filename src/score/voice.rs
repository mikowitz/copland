use crate::container::Containable;
use crate::error::Error;
use crate::lilypond::{context_signature, delimiters, format_contents, ToLilypond};

#[derive(Debug, Clone)]
pub struct Voice {
    contents: Vec<Containable>,
    name: Option<String>,
    simultaneous: bool,
}

impl Voice {
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

impl ToLilypond for Voice {
    fn to_lilypond(&self) -> Result<String, Error> {
        let (open, close) = delimiters(self.simultaneous);
        Ok(format!(
            "{} {open}\n{}\n{close}",
            context_signature(&self.name, "Voice"),
            format_contents(&self.contents)
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn without_name() {
        let voice = Voice::new(vec![Rest::new(Duration::new(1, 4)).unwrap().into()]);

        assert_eq!(voice.contents.len(), 1);
        assert_eq!(voice.name, None);
        assert!(!voice.simultaneous);

        assert_eq!(
            voice.to_lilypond().unwrap(),
            r#"
\new Voice {
  r4
}
        "#
            .trim()
        )
    }

    #[test]
    fn with_name() {
        let mut voice = Voice::new(vec![Rest::new(Duration::new(1, 4)).unwrap().into()]);
        voice.set_name("Voice One");

        assert_eq!(voice.contents.len(), 1);
        assert_eq!(voice.name, Some("Voice One".to_string()));
        assert!(!voice.simultaneous);

        assert_eq!(
            voice.to_lilypond().unwrap(),
            r#"
\context Voice = "Voice One" {
  r4
}
        "#
            .trim()
        )
    }

    #[test]
    fn simultaneous() {
        let mut voice = Voice::new(vec![Rest::new(Duration::new(1, 4)).unwrap().into()]);
        voice.set_name("Voice One");
        voice.set_simultaneous(true);

        assert_eq!(voice.contents.len(), 1);
        assert_eq!(voice.name, Some("Voice One".to_string()));
        assert!(voice.simultaneous);

        assert_eq!(
            voice.to_lilypond().unwrap(),
            r#"
\context Voice = "Voice One" <<
  r4
>>
        "#
            .trim()
        )
    }
}
