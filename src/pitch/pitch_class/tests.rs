use super::PitchClass;
use crate::pitch::Accidental::*;
use crate::pitch::DiatonicPitchClass::*;

#[test]
fn semitones() {
    let c = PitchClass::new(C, Natural);
    assert_eq!(c.semitones(), 0.);

    let bss = PitchClass::new(B, DoubleSharp);
    assert_eq!(bss.semitones(), 1.);

    let cqf = PitchClass::new(C, QuarterFlat);
    assert_eq!(cqf.semitones(), 11.5);

    let etqs = PitchClass::new(E, ThreeQuarterSharp);
    assert_eq!(etqs.semitones(), 5.5);
}

#[test]
fn to_string() {
    let c = PitchClass::new(C, Natural);
    assert_eq!(c.to_string(), "c");

    let bss = PitchClass::new(B, DoubleSharp);
    assert_eq!(bss.to_string(), "bss");

    let cqf = PitchClass::new(C, QuarterFlat);
    assert_eq!(cqf.to_string(), "cqf");

    let etqs = PitchClass::new(E, ThreeQuarterSharp);
    assert_eq!(etqs.to_string(), "etqs");
}
