use super::IntervalClass;
use crate::interval::interval_size::IntervalSize::*;
use crate::interval::polarity::Polarity::*;
use crate::interval::quality::Quality::*;
use crate::interval::quartertone::Quartertone::*;

#[test]
fn new() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();

    assert_eq!(p1.size, Unison);
    assert_eq!(p1.quality, Perfect);
    assert_eq!(p1.polarity, None);
}

#[test]
fn new_with_invalid_interval() {
    assert!(IntervalClass::new(Perfect, Unison).is_ok());
    assert!(IntervalClass::new(Major, Unison).is_err());
    assert!(IntervalClass::new(Minor, Unison).is_err());
    assert!(IntervalClass::new(Augmented(1), Unison).is_ok());
    assert!(IntervalClass::new(Diminished(1), Unison).is_ok());

    assert!(IntervalClass::new(Perfect, Second).is_err());
    assert!(IntervalClass::new(Major, Second).is_ok());
    assert!(IntervalClass::new(Minor, Second).is_ok());
    assert!(IntervalClass::new(Augmented(1), Second).is_ok());
    assert!(IntervalClass::new(Diminished(1), Second).is_ok());

    assert!(IntervalClass::new(Perfect, Third).is_err());
    assert!(IntervalClass::new(Major, Third).is_ok());
    assert!(IntervalClass::new(Minor, Third).is_ok());
    assert!(IntervalClass::new(Augmented(1), Third).is_ok());
    assert!(IntervalClass::new(Diminished(1), Third).is_ok());

    assert!(IntervalClass::new(Perfect, Fourth).is_ok());
    assert!(IntervalClass::new(Major, Fourth).is_err());
    assert!(IntervalClass::new(Minor, Fourth).is_err());
    assert!(IntervalClass::new(Augmented(1), Fourth).is_ok());
    assert!(IntervalClass::new(Diminished(1), Fourth).is_ok());

    assert!(IntervalClass::new(Perfect, Fifth).is_ok());
    assert!(IntervalClass::new(Major, Fifth).is_err());
    assert!(IntervalClass::new(Minor, Fifth).is_err());
    assert!(IntervalClass::new(Augmented(1), Fifth).is_ok());
    assert!(IntervalClass::new(Diminished(1), Fifth).is_ok());

    assert!(IntervalClass::new(Perfect, Sixth).is_err());
    assert!(IntervalClass::new(Major, Sixth).is_ok());
    assert!(IntervalClass::new(Minor, Sixth).is_ok());
    assert!(IntervalClass::new(Augmented(1), Sixth).is_ok());
    assert!(IntervalClass::new(Diminished(1), Sixth).is_ok());

    assert!(IntervalClass::new(Perfect, Seventh).is_err());
    assert!(IntervalClass::new(Major, Seventh).is_ok());
    assert!(IntervalClass::new(Minor, Seventh).is_ok());
    assert!(IntervalClass::new(Augmented(1), Seventh).is_ok());
    assert!(IntervalClass::new(Diminished(1), Seventh).is_ok());

    assert!(IntervalClass::new(Perfect, Octave).is_ok());
    assert!(IntervalClass::new(Major, Octave).is_err());
    assert!(IntervalClass::new(Minor, Octave).is_err());
    assert!(IntervalClass::new(Augmented(1), Octave).is_ok());
    assert!(IntervalClass::new(Diminished(1), Octave).is_ok());
}

#[test]
fn only_perfect_octaves_arent_reduced() {
    assert_eq!(
        IntervalClass::new(Perfect, Octave),
        IntervalClass::new(Perfect, Octave)
    );
    assert_eq!(
        IntervalClass::new(Augmented(2), Octave),
        IntervalClass::new(Augmented(2), Unison)
    );
    assert_eq!(
        IntervalClass::new(Diminished(1), Octave),
        IntervalClass::new(Diminished(1), Unison)
    );
}

#[test]
fn negate() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();
    let neg_m2 = -IntervalClass::new(Minor, Second).unwrap();

    assert_eq!(-p1, p1);
    assert_eq!(neg_m2.polarity, Some(Negative));
}

#[test]
fn quarter_sharp() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();
    let pqs1 = p1.quarter_sharp();

    assert_eq!(pqs1.quartertone, Some(QuarterSharp));
    assert_eq!(pqs1.polarity, Some(Positive));
}

#[test]
fn quarter_flat() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();
    let pqs1 = p1.quarter_flat();

    assert_eq!(pqs1.quartertone, Some(QuarterFlat));
    assert_eq!(pqs1.polarity, Some(Positive));
}

#[test]
fn semitones() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();
    assert_eq!(p1.semitones(), 0.);

    let neg_maj2 = -IntervalClass::new(Major, Second).unwrap();
    assert_eq!(neg_maj2.semitones(), -2.);

    let neg_pqs4 = -IntervalClass::new(Perfect, Fourth).unwrap().quarter_sharp();
    assert_eq!(neg_pqs4.semitones(), -5.5);

    let aug3 = IntervalClass::new(Augmented(1), Third).unwrap();
    assert_eq!(aug3.semitones(), 5.);
}

#[test]
fn is_perfect_octave() {
    let p8 = IntervalClass::new(Perfect, Octave).unwrap();
    assert!(p8.is_perfect_octave());

    let pq8 = p8.quarter_sharp();
    assert!(!pq8.is_perfect_octave());
}

#[test]
fn to_string() {
    let p1 = IntervalClass::new(Perfect, Unison).unwrap();
    assert_eq!(p1.to_string(), "P1");

    let mq3 = -IntervalClass::new(Minor, Third).unwrap().quarter_flat();
    assert_eq!(mq3.to_string(), "-m~3");
}
