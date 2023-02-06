use super::{Interval, IntervalSize::*, Polarity::*, Quality::*, Quartertone::*};

#[test]
fn new() {
    let interval = Interval::new(Perfect, 1);

    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 0);
    assert!(interval.polarity.is_none());

    let interval = Interval::new(Perfect, 8);
    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.size, Octave);
    assert_eq!(interval.octaves, 0);
    assert_eq!(interval.polarity, Some(Positive));

    let interval = Interval::new(Augmented(1), 8);
    assert_eq!(interval.interval_class.quality, Augmented(1));
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 1);
    assert_eq!(interval.polarity, Some(Positive));

    let interval = Interval::new(Major, -2);
    assert_eq!(interval.interval_class.quality, Major);
    assert_eq!(interval.interval_class.size, Second);
    assert_eq!(interval.interval_class.polarity, Some(Negative));
    assert_eq!(interval.octaves, 0);
    assert_eq!(interval.polarity, Some(Negative));
}

#[test]
fn negate() {
    let interval = Interval::new(Major, 10);

    let neg_interval = -interval;

    assert_eq!(neg_interval.interval_class.polarity, Some(Negative));
    assert_eq!(neg_interval.interval_class.size, Third);
    assert_eq!(neg_interval.polarity, Some(Negative));
    assert_eq!(neg_interval.octaves, 1);
}

#[test]
fn quarter_sharp() {
    let interval = Interval::new(Minor, 10).quarter_sharp();

    assert_eq!(interval.interval_class.quartertone, Some(QuarterSharp));

    let interval = Interval::new(Perfect, 8).quarter_sharp();
    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 1);

    let interval = -Interval::new(Perfect, 15).quarter_sharp();
    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 2);
    assert_eq!(interval.polarity, Some(Negative));
}

#[test]
fn quarter_flat() {
    let interval = Interval::new(Perfect, 11).quarter_flat();

    assert_eq!(interval.interval_class.quartertone, Some(QuarterFlat));

    let interval = Interval::new(Perfect, 8).quarter_flat();
    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.quartertone, Some(QuarterFlat));
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 1);

    let interval = -Interval::new(Perfect, 15).quarter_flat();
    assert_eq!(interval.interval_class.quality, Perfect);
    assert_eq!(interval.interval_class.size, Unison);
    assert_eq!(interval.octaves, 2);
    assert_eq!(interval.polarity, Some(Negative));
}

#[test]
fn semitones() {
    let interval = Interval::new(Major, 9);
    let neg = -interval;

    assert_eq!(interval.semitones(), 14.);
    assert_eq!(neg.semitones(), -14.);

    let aug8 = Interval::new(Augmented(2), 8);
    assert_eq!(aug8.semitones(), 14.);
}

#[test]
fn staff_spaces() {
    let interval = Interval::new(Major, 9);
    let neg = -interval;

    assert_eq!(interval.staff_spaces(), 8);
    assert_eq!(neg.staff_spaces(), -8);
}

#[test]
fn to_string() {
    let interval = Interval::new(Major, 9);
    let neg = -interval;

    assert_eq!(interval.to_string(), "+M9");
    assert_eq!(neg.to_string(), "-M9");

    let aug8 = Interval::new(Augmented(2), 8);
    assert_eq!(aug8.to_string(), "+AA8");
}
