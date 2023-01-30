use super::Duration;

#[test]
fn new_duration() {
    let d = Duration::new(1, 4);

    assert_eq!(d.as_tuple(), (1, 4));
    assert_eq!(d.as_float(), 0.25);
}

#[test]
fn new_reduced_duration() {
    let d = Duration::new(2, 4);

    assert_eq!(d.as_tuple(), (1, 2));
}

#[test]
fn add_durations() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert_eq!(d1 + d2, Duration::new(7, 12));
}

#[test]
fn is_printable() {
    let d1 = Duration::new(1, 4);
    let d2 = Duration::new(1, 3);

    assert!(d1.is_printable());
    assert!(!d2.is_printable());
}
