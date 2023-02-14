use crate::duration::Duration;
use crate::interval::Interval;
use crate::leaf::Leaf;
use crate::notehead::Notehead;
use crate::pitch::Pitch;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    notehead: Notehead,
    written_duration: Duration,
}

impl Note {
    pub const fn new(written_pitch: Pitch, written_duration: Duration) -> Self {
        let notehead = Notehead::new(written_pitch);
        Self {
            notehead,
            written_duration,
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
    fn written_duration(&self) -> Self::Duration {
        self.written_duration
    }

    fn to_note(&self) -> Self {
        *self
    }

    fn to_rest(&self) -> Self::Rest {
        Self::Rest::new(self.written_duration)
    }

    fn to_spacer(&self) -> Self::Spacer {
        Self::Spacer::new(self.written_duration)
    }

    fn to_chord(&self) -> Self::Chord {
        Self::Chord::new(&[self.written_pitch()], self.written_duration)
    }
}

impl ToLilypond for Note {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!(
                "{}{duration_lilypond}",
                self.notehead.to_lilypond().unwrap(),
            )),
            Err(err) => Err(err),
        }
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
