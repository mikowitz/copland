use crate::duration::Duration;
use crate::leaf::Leaf;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rest {
    written_duration: Duration,
}

impl Rest {
    pub const fn new(written_duration: Duration) -> Self {
        Self { written_duration }
    }
}

impl Leaf for Rest {
    fn written_duration(&self) -> Self::Duration {
        self.written_duration
    }

    fn to_note(&self) -> Self::Note {
        crate::note::Note::new(crate::pitch::C4, self.written_duration)
    }

    fn to_rest(&self) -> Self::Rest {
        *self
    }

    fn to_chord(&self) -> Self::Chord {
        crate::chord::Chord::new(&[crate::pitch::C4], self.written_duration)
    }

    fn to_spacer(&self) -> Self::Spacer {
        crate::spacer::Spacer::new(self.written_duration)
    }
}

impl ToLilypond for Rest {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!("r{duration_lilypond}")),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::Duration;

    fn rest() -> Rest {
        Rest::new(Duration::new(3, 4))
    }

    #[test]
    fn to_lilypond() {
        let rest = Rest::new(Duration::new(3, 4));

        assert_eq!(rest.to_lilypond().unwrap(), "r2.");
    }

    #[test]
    fn to_rest() {
        assert_eq!(rest().to_rest(), rest());
    }

    #[test]
    fn to_spacer() {
        assert_eq!(rest().to_spacer().to_lilypond().unwrap(), "s2.");
    }

    #[test]
    fn to_chord() {
        assert_eq!(rest().to_chord().to_lilypond().unwrap(), "<\n  c'\n>2.");
    }

    #[test]
    fn to_note() {
        assert_eq!(rest().to_note().to_lilypond().unwrap(), "c'2.");
    }
}
