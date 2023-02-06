use super::Accidental::*;
use super::DiatonicPitchClass::*;
use super::Pitch;
use super::PitchClass;
use crate::interval::{Interval, Quality::*};

#[test]
fn semitones() {
    let c4 = Pitch::new(PitchClass::new(C, Natural), 4);
    assert_eq!(c4.semitones(), 0.);

    let fs3 = Pitch::new(PitchClass::new(F, Sharp), 3);
    assert_eq!(fs3.semitones(), -6.);

    let cqf4 = Pitch::new(PitchClass::new(C, QuarterFlat), 4);
    assert_eq!(cqf4.semitones(), -0.5);
}

#[test]
fn to_string() {
    let c4 = Pitch::new(PitchClass::new(C, Natural), 4);
    assert_eq!(c4.to_string(), "c'");

    let fs3 = Pitch::new(PitchClass::new(F, Sharp), 3);
    assert_eq!(fs3.to_string(), "fs");

    let cqf4 = Pitch::new(PitchClass::new(C, QuarterFlat), 4);
    assert_eq!(cqf4.to_string(), "cqf'");

    let gtqs2 = Pitch::new(PitchClass::new(G, ThreeQuarterSharp), 2);
    assert_eq!(gtqs2.to_string(), "gtqs,");

    let af5 = Pitch::new(PitchClass::new(A, Flat), 5);
    assert_eq!(af5.to_string(), "af''");
}

#[test]
fn transpose() {
    let c4 = Pitch::new(PitchClass::new(C, Natural), 4);
    let p1 = Interval::new(Perfect, 1);
    let minor3 = Interval::new(Minor, 3);
    let neg_p5 = -Interval::new(Perfect, 5);

    assert_eq!(c4.transpose(p1), c4);
    assert_eq!(
        c4.transpose(minor3),
        Pitch::new(PitchClass::new(E, Flat), 4)
    );
    assert_eq!(
        c4.transpose(neg_p5),
        Pitch::new(PitchClass::new(F, Natural), 3)
    );

    let dqs4 = Pitch::new(PitchClass::new(D, QuarterSharp), 4);
    assert_eq!(dqs4.transpose(p1), dqs4);
    assert_eq!(
        dqs4.transpose(minor3),
        Pitch::new(PitchClass::new(F, QuarterSharp), 4)
    );
    assert_eq!(
        dqs4.transpose(neg_p5),
        Pitch::new(PitchClass::new(G, QuarterSharp), 3)
    );

    let aug8 = Interval::new(Augmented(1), 8);

    assert_eq!(c4.transpose(aug8), Pitch::new(PitchClass::new(C, Sharp), 5));

    let ess4 = Pitch::new(PitchClass::new(E, DoubleSharp), 4);
    let a1 = Interval::new(Augmented(1), 1);

    assert_eq!(
        ess4.transpose(a1),
        Pitch::new(PitchClass::new(F, DoubleSharp), 4)
    );
}
