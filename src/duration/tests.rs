#[cfg(test)]
mod regression_data_loaders;

use super::Duration;
use crate::error::Error;
use crate::to_lilypond::ToLilypond;
use itertools::Itertools;
use regression_data_loaders::*;

#[test]
fn new_duration() {
    let d = Duration::new(1, 4);

    assert_eq!(d.as_tuple(), (1, 4));
    assert_eq!(d.as_float(), 0.25);
}

#[test]
#[ignore = "regression"]
fn new_duration_regression() {
    let data = aw!(load_duration_new());
    let res = data
        .iter()
        .map(|data| Duration::new(data[0], data[1]) == Duration::new(data[2], data[3]))
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn new_reduced_duration() {
    let d = Duration::new(2, 4);

    assert_eq!(d.as_tuple(), (1, 2));
}

#[test]
fn new_duration_with_corrected_polarity() {
    let mut d = Duration::new(-1, 4);
    assert_eq!(d.as_tuple(), (-1, 4));

    d = Duration::new(-1, -4);
    assert_eq!(d.as_tuple(), (1, 4));

    d = Duration::new(1, -4);
    assert_eq!(d.as_tuple(), (-1, 4));
}

#[test]
fn add_durations() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert_eq!(d1 + d2, Duration::new(7, 12));
}

#[test]
#[ignore = "regression"]
fn add_regression() {
    let data = aw!(load_duration_add());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = Duration::new(data[2], data[3]);
            let sum = Duration::new(data[4], data[5]);
            lhs + rhs == sum
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn subtract() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert_eq!(d1 - d2, Duration::new(-1, 12));
    assert_eq!(d2 - d1, Duration::new(1, 12));
}

#[test]
#[ignore = "regression"]
fn subtract_regression() {
    let data = aw!(load_duration_subtract());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = Duration::new(data[2], data[3]);
            let difference = Duration::new(data[4], data[5]);
            lhs - rhs == difference
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn multiply() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert_eq!(d1 * d2, Duration::new(1, 12));

    assert_eq!(d1 * 3, Duration::new(3, 4));
    assert_eq!(d1 * 2. as i32, Duration::new(1, 2));
}

#[test]
#[ignore = "regression"]
fn multiply_regression() {
    let data = aw!(load_duration_multiply());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = Duration::new(data[2], data[3]);
            let product = Duration::new(data[4], data[5]);
            lhs * rhs == product
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
#[ignore = "regression"]
fn multiply_by_int_regression() {
    let data = aw!(load_duration_multiply_by_int());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = data[2];
            let product = Duration::new(data[3], data[4]);
            lhs * rhs == product
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn divide() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert_eq!(d1 / d2, Duration::new(3, 4));

    assert_eq!(d1 / 3, Duration::new(1, 12));
    assert_eq!(d2 / 2. as i32, Duration::new(1, 6));
}

#[test]
#[ignore = "regression"]
fn divide_regression() {
    let data = aw!(load_duration_divide());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = Duration::new(data[2], data[3]);
            let quotient = Duration::new(data[4], data[5]);
            lhs / rhs == quotient
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
#[ignore = "regression"]
fn divide_by_int_regression() {
    let data = aw!(load_duration_divide_by_int());
    let res = data
        .iter()
        .map(|data| {
            let lhs = Duration::new(data[0], data[1]);
            let rhs = data[2];
            let quotient = Duration::new(data[3], data[4]);
            lhs / rhs == quotient
        })
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn is_printable() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert!(d1.is_printable());
    assert!(!d2.is_printable());
}

#[test]
#[ignore = "regression"]
fn is_printable_regression() {
    let data = aw!(load_duration_printable());
    let res = data
        .iter()
        .map(|data| Duration::new(data.0, data.1).is_printable() == data.2)
        .unique()
        .collect::<Vec<bool>>();

    assert_eq!(res, vec![true]);
}

#[test]
fn is_negative() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(-1, 4);

    assert!(!d1.is_negative());
    assert!(d2.is_negative());
    assert!(!Duration::new(0, 1).is_negative());
}

#[test]
fn negate() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(-1, 4);

    assert_eq!(-d1, d2);
    assert_eq!(-d2, d1);
}

#[test]
fn abs() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(-1, 4);

    assert_eq!(d1.abs(), d1);
    assert_eq!(d2.abs(), d1);
}

#[test]
fn to_lilypond() {
    let mut d = Duration::new(1, 4);
    assert_eq!(d.to_lilypond().unwrap(), "4");

    d = Duration::new(3, 8);
    assert_eq!(d.to_lilypond().unwrap(), "4.");

    d = Duration::new(7, 32);
    assert_eq!(d.to_lilypond().unwrap(), "8..");

    d = Duration::new(15, 1);
    assert_eq!(d.to_lilypond().unwrap(), "\\maxima...")
}

#[test]
fn to_lilypond_for_unprintable_durations() {
    let mut d = Duration::new(23, 1);
    assert_eq!(d.to_lilypond(), Err(Error::UnprintableDuration(d)));

    d = Duration::new(5, 8);
    assert_eq!(d.to_lilypond(), Err(Error::UnprintableDuration(d)));

    d = Duration::new(1, 5);
    assert_eq!(d.to_lilypond(), Err(Error::UnprintableDuration(d)));
}

#[test]
fn equal_or_shorter_printable_duration() {
    let d = Duration::new(5, 8);
    let d2 = Duration::new(1, 2);

    assert_eq!(d.equal_or_shorter_printable_duration(), d2);
    assert_eq!(d2.equal_or_shorter_printable_duration(), d2);
}

#[test]
fn equal_or_greater_printable_duration() {
    let d = Duration::new(5, 8);
    let d2 = Duration::new(3, 4);

    assert_eq!(d.equal_or_greater_printable_duration(), d2);
    assert_eq!(d2.equal_or_greater_printable_duration(), d2);
}

#[test]
fn to_printable_duration_list() {
    let d = Duration::new(1, 4);
    assert_eq!(d.to_printable_duration_list(), vec![Duration::new(1, 4)]);

    let d2 = Duration::new(5, 8);
    assert_eq!(
        d2.to_printable_duration_list(),
        vec![Duration::new(1, 2), Duration::new(1, 8)]
    );

    let d3 = Duration::new(-1, 4);
    assert_eq!(d3.to_printable_duration_list(), vec![] as Vec<Duration>);
}
