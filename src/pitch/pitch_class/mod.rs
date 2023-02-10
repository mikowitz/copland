use super::{Accidental, DiatonicPitchClass};
use std::fmt;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PitchClass {
    diatonic_pitch_class: DiatonicPitchClass,
    accidental: Accidental,
}

impl PitchClass {
    pub const fn new(diatonic_pitch_class: DiatonicPitchClass, accidental: Accidental) -> Self {
        Self {
            diatonic_pitch_class,
            accidental,
        }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_float(&self) -> f32 {
        self.diatonic_pitch_class as i32 as f32 + self.accidental.as_float()
    }

    #[must_use]
    pub const fn diatonic_pitch_class_index(&self) -> i32 {
        self.diatonic_pitch_class.index()
    }

    #[must_use]
    pub fn semitones(&self) -> f32 {
        match self.as_float() {
            s if s < 0. => s + 12.,
            s if s >= 12. => s - 12.,
            s => s,
        }
    }

    pub const fn diatonic_pitch_class(self) -> DiatonicPitchClass {
        self.diatonic_pitch_class
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.diatonic_pitch_class, self.accidental)
    }
}

#[cfg(test)]
mod tests;
