use crate::duration::Duration;
use crate::error::Error;
use crate::interval::Interval;
use crate::leaf::Leaf;
use crate::lilypond::ToLilypond;
use crate::notehead::Notehead;
use crate::pitch::Pitch;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    notehead: Notehead,
    written_duration: Duration,
}

impl Note {
    /// Returns a new `Note` from the given `Pitch` and `Duration`
    ///
    /// # Errors
    ///
    /// Returns an `UnprintableDuration` error if the given `Duration` cannot be rendered by a
    /// single notehead in a score.
    pub fn new(written_pitch: Pitch, written_duration: Duration) -> Result<Self, Error> {
        if written_duration.is_printable() {
            let notehead = Notehead::new(written_pitch);
            Ok(Self {
                notehead,
                written_duration,
            })
        } else {
            Err(Error::UnprintableDuration(written_duration))
        }
    }

    pub const fn written_pitch(&self) -> Pitch {
        self.notehead.written_pitch()
    }

    pub fn transpose(&mut self, interval: Interval) {
        self.notehead.transpose(interval);
    }
}

impl Leaf for Note {
    fn written_duration(&self) -> crate::duration::Duration {
        self.written_duration
    }

    fn to_note(&self) -> Self {
        *self
    }

    fn to_rest(&self) -> crate::rest::Rest {
        crate::rest::Rest::new(self.written_duration).unwrap()
    }

    fn to_spacer(&self) -> crate::spacer::Spacer {
        crate::spacer::Spacer::new(self.written_duration).unwrap()
    }

    fn to_chord(&self) -> crate::chord::Chord {
        crate::chord::Chord::new(&[self.written_pitch()], self.written_duration).unwrap()
    }
}

impl ToLilypond for Note {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        Ok(format!(
            "{}{}",
            self.notehead.to_lilypond().unwrap(),
            self.written_duration.to_lilypond().unwrap()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::*;
    use crate::pitch::*;

    fn note() -> Note {
        Note::new(
            Pitch::new(PitchClass::new(DiatonicPitchClass::E, Accidental::Flat), 2),
            Duration::new(1, 8),
        )
        .unwrap()
    }

    #[test]
    fn new() {
        assert!(Note::new(note().written_pitch(), Duration::new(1, 5)).is_err());
    }

    #[test]
    fn to_lilypond() {
        assert_eq!(note().to_lilypond().unwrap(), "ef,8");
    }

    #[test]
    fn to_rest() {
        assert_eq!(note().to_rest().to_lilypond().unwrap(), "r8");
    }

    #[test]
    fn to_spacer() {
        assert_eq!(note().to_spacer().to_lilypond().unwrap(), "s8");
    }

    #[test]
    fn to_chord() {
        assert_eq!(note().to_chord().to_lilypond().unwrap(), "<\n  ef,\n>8");
    }

    #[test]
    fn to_note() {
        assert_eq!(note().to_note(), note());
    }

    #[test]
    fn transpose() {
        let mut note = note();
        note.transpose(crate::interval::Interval::new(
            crate::interval::Quality::Major,
            3,
        ));
        assert_eq!(note.to_lilypond().unwrap(), "g,8");
    }
}
