use crate::interval::Interval;
use crate::pitch::Pitch;
use crate::to_lilypond::ToLilypond;

mod accidental_display;
pub use accidental_display::AccidentalDisplay;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Notehead {
    written_pitch: Pitch,
    accidental_display: Option<AccidentalDisplay>,
}

impl Notehead {
    pub const fn new(written_pitch: Pitch) -> Self {
        Self {
            written_pitch,
            accidental_display: None,
        }
    }

    pub fn transpose(&mut self, interval: Interval) {
        self.written_pitch = self.written_pitch().transpose(interval);
    }

    pub const fn forced(self) -> Self {
        Self {
            accidental_display: Some(AccidentalDisplay::Forced),
            ..self
        }
    }

    pub const fn cautionary(self) -> Self {
        Self {
            accidental_display: Some(AccidentalDisplay::Cautionary),
            ..self
        }
    }

    pub const fn neutral(self) -> Self {
        Self {
            accidental_display: None,
            ..self
        }
    }

    pub const fn written_pitch(self) -> Pitch {
        self.written_pitch
    }
}

impl ToLilypond for Notehead {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        let accidental_display = self
            .accidental_display
            .map_or(String::new(), |ad| ad.to_string());
        Ok(format!("{}{}", self.written_pitch, accidental_display))
    }
}

#[cfg(test)]
mod tests {
    use crate::to_lilypond::ToLilypond;
    use crate::{
        interval::{Interval, Quality},
        pitch::*,
    };

    use super::Notehead;

    #[test]
    fn to_lilypond() {
        let pitch = Pitch::new(
            PitchClass::new(DiatonicPitchClass::C, Accidental::Natural),
            4,
        );

        let notehead = Notehead::new(pitch);
        assert_eq!(notehead.to_lilypond().unwrap(), "c'");

        let notehead2 = Notehead::new(pitch).forced();
        assert_eq!(notehead2.to_lilypond().unwrap(), "c'!");

        let notehead2 = Notehead::new(pitch).cautionary();
        assert_eq!(notehead2.to_lilypond().unwrap(), "c'?");

        let notehead3 = notehead2.neutral();
        assert_eq!(notehead3.to_lilypond().unwrap(), "c'");
    }

    #[test]
    fn transpose() {
        let pitch = Pitch::new(
            PitchClass::new(DiatonicPitchClass::C, Accidental::Natural),
            4,
        );

        let mut notehead = Notehead::new(pitch);

        notehead.transpose(Interval::new(Quality::Perfect, 4));

        assert_eq!(notehead.to_lilypond().unwrap(), "f'");
    }
}
