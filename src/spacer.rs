use crate::duration::Duration;
use crate::error::Error;
use crate::leaf::Leaf;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spacer {
    written_duration: Duration,
}

impl Spacer {
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

impl Leaf for Spacer {
    fn written_duration(&self) -> Self::Duration {
        self.written_duration
    }

    fn to_note(&self) -> Self::Note {
        crate::note::Note::new(crate::pitch::C4, self.written_duration).unwrap()
    }

    fn to_rest(&self) -> Self::Rest {
        crate::rest::Rest::new(self.written_duration).unwrap()
    }

    fn to_chord(&self) -> Self::Chord {
        crate::chord::Chord::new(&[crate::pitch::C4], self.written_duration).unwrap()
    }

    fn to_spacer(&self) -> Self::Spacer {
        *self
    }
}

impl ToLilypond for Spacer {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!("s{duration_lilypond}")),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::Duration;

    fn spacer() -> Spacer {
        Spacer::new(Duration::new(7, 16)).unwrap()
    }

    #[test]
    fn to_lilypond() {
        assert_eq!(spacer().to_lilypond().unwrap(), "s4..");
    }

    #[test]
    fn to_rest() {
        assert_eq!(spacer().to_rest().to_lilypond().unwrap(), "r4..");
    }

    #[test]
    fn to_spacer() {
        assert_eq!(spacer().to_spacer(), spacer());
    }

    #[test]
    fn to_chord() {
        assert_eq!(spacer().to_chord().to_lilypond().unwrap(), "<\n  c'\n>4..");
    }

    #[test]
    fn to_note() {
        assert_eq!(spacer().to_note().to_lilypond().unwrap(), "c'4..");
    }
}
