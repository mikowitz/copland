use crate::duration::Duration;
use crate::error::Error;
use crate::leaf::Leaf;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rest {
    written_duration: Duration,
}

impl Rest {
    /// Returns a new `Spacer` from the given `Duration`.
    ///
    /// # Errors
    ///
    /// Returns an `UnprintableDuration` error if the given `Duration` cannot be rendered by a
    /// single notehead in a score.
    pub fn new(written_duration: Duration) -> Result<Self, Error> {
        if written_duration.is_printable() {
            Ok(Self { written_duration })
        } else {
            Err(Error::UnprintableDuration(written_duration))
        }
    }
}

impl Leaf for Rest {
    fn written_duration(&self) -> Self::Duration {
        self.written_duration
    }

    fn to_note(&self) -> Self::Note {
        crate::note::Note::new(crate::pitch::C4, self.written_duration).unwrap()
    }

    fn to_rest(&self) -> Self::Rest {
        *self
    }

    fn to_chord(&self) -> Self::Chord {
        crate::chord::Chord::new(&[crate::pitch::C4], self.written_duration).unwrap()
    }

    fn to_spacer(&self) -> Self::Spacer {
        crate::spacer::Spacer::new(self.written_duration).unwrap()
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
        Rest::new(Duration::new(3, 4)).unwrap()
    }

    #[test]
    fn to_lilypond() {
        assert_eq!(rest().to_lilypond().unwrap(), "r2.");
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
