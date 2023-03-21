mod accidental;
mod diatonic_pitch_class;
mod pitch_class;

pub use accidental::Accidental;
pub use diatonic_pitch_class::DiatonicPitchClass;
pub use pitch_class::PitchClass;

use crate::has_semitones::HasSemitones;

type Octave = i32;

impl HasSemitones for Octave {
    fn semitones(&self) -> f32 {
        (self - 4) as f32 * 12.
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pitch_class: PitchClass,
    octave: Octave,
}

impl Pitch {
    pub fn new(pitch_class: PitchClass, octave: Octave) -> Self {
        Self {
            pitch_class,
            octave,
        }
    }
}

impl HasSemitones for Pitch {
    fn semitones(&self) -> f32 {
        self.pitch_class.semitones() + self.octave.semitones()
    }
}

#[cfg(test)]
mod tests {
    use super::{Accidental::*, DiatonicPitchClass::*, HasSemitones, Pitch, PitchClass};

    #[test]
    fn semitones() {
        let c4 = Pitch::new(PitchClass::new(C, Natural), 4);
        assert_eq!(c4.semitones(), 0.);

        let ef3 = Pitch::new(PitchClass::new(E, Flat), 3);
        assert_eq!(ef3.semitones(), -9.);

        let fqs6 = Pitch::new(PitchClass::new(F, QuarterSharp), 6);
        assert_eq!(fqs6.semitones(), 29.5);
    }
}
