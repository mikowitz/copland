use crate::duration::Duration;
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
}

impl Leaf for Note {}

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

    #[test]
    fn to_lilypond() {
        let note = Note::new(
            Pitch::new(PitchClass::new(DiatonicPitchClass::E, Accidental::Flat), 2),
            Duration::new(1, 8),
        );

        assert_eq!(note.to_lilypond().unwrap(), "ef,8");
    }
}
