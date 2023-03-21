use super::{Accidental, DiatonicPitchClass};
use crate::has_semitones::HasSemitones;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PitchClass {
    diatonic_pitch_class: DiatonicPitchClass,
    accidental: Accidental,
}

impl PitchClass {
    pub fn new(diatonic_pitch_class: DiatonicPitchClass, accidental: Accidental) -> Self {
        Self {
            diatonic_pitch_class,
            accidental,
        }
    }

    pub fn to_pair(&self) -> (DiatonicPitchClass, Accidental) {
        (self.diatonic_pitch_class, self.accidental)
    }

    pub fn semitones(&self) -> f32 {
        match self.diatonic_pitch_class.semitones() + self.accidental.semitones() {
            s if s < 0. => s + 12.,
            s if s >= 12. => s - 12.,
            s => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Accidental::*, DiatonicPitchClass::*, PitchClass};

    #[test]
    fn new() {
        let pitch_class = PitchClass::new(C, Natural);

        assert_eq!(pitch_class.to_pair(), (C, Natural));
    }

    #[test]
    fn semitones() {
        assert_eq!(PitchClass::new(C, Natural).semitones(), 0.);
        assert_eq!(PitchClass::new(E, Flat).semitones(), 3.);
        assert_eq!(PitchClass::new(C, DoubleFlat).semitones(), 10.);
        assert_eq!(PitchClass::new(B, ThreeQuarterSharp).semitones(), 0.5);
    }
}
