use crate::container::Containable;
use crate::error::Error;
use crate::lilypond::ToLilypond;

#[derive(Debug, Clone)]
pub struct Multiplier(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Tuplet {
    multiplier: Multiplier,
    contents: Vec<Containable>,
}

impl Tuplet {
    #[must_use]
    pub fn new(multiplier: Multiplier, contents: Vec<Containable>) -> Self {
        Self {
            multiplier,
            contents,
        }
    }

    #[must_use]
    pub fn contents(&self) -> &[Containable] {
        &self.contents
    }
}

impl ToLilypond for Tuplet {
    fn to_lilypond(&self) -> Result<String, Error> {
        Ok(format!(
            "\\tuplet {}/{} {{\n{}\n}}",
            self.multiplier.1,
            self.multiplier.0,
            crate::lilypond::format_contents(&self.contents)
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn note(dpc: DiatonicPitchClass) -> Note {
        Note::new(
            Pitch::new(PitchClass::new(dpc, Accidental::Natural), 4),
            Duration::new(1, 8),
        )
        .unwrap()
    }

    #[test]
    fn new() {
        let tuplet = Tuplet::new(
            Multiplier(2, 3),
            vec![
                note(DiatonicPitchClass::C).into(),
                note(DiatonicPitchClass::D).into(),
                note(DiatonicPitchClass::E).into(),
            ],
        );

        assert_eq!(
            tuplet.to_lilypond().unwrap(),
            r#"
\tuplet 3/2 {
  c'8
  d'8
  e'8
}
        "#
            .trim()
        );
    }
}
