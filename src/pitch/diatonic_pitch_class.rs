use crate::has_semitones::HasSemitones;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiatonicPitchClass {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl HasSemitones for DiatonicPitchClass {
    fn semitones(&self) -> f32 {
        *self as i32 as f32
    }
}
