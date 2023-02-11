use super::Chord;
use crate::duration::Duration;
use crate::pitch::*;
use crate::to_lilypond::ToLilypond;

fn c_major() -> Vec<Pitch> {
    vec![
        Pitch::new(
            PitchClass::new(DiatonicPitchClass::C, Accidental::Natural),
            4,
        ),
        Pitch::new(
            PitchClass::new(DiatonicPitchClass::E, Accidental::Natural),
            4,
        ),
        Pitch::new(
            PitchClass::new(DiatonicPitchClass::G, Accidental::Natural),
            4,
        ),
    ]
}

#[test]
fn written_pitches() {
    let pitches = c_major();
    let chord = Chord::new(&pitches, Duration::new(1, 4));

    assert_eq!(chord.written_pitches(), pitches);
}

#[test]
fn insert_pitch() {
    let mut chord = Chord::new(&c_major(), Duration::new(1, 4));

    chord.insert(Pitch::new(
        PitchClass::new(DiatonicPitchClass::B, Accidental::Flat),
        3,
    ));

    assert_eq!(
        chord.to_lilypond().unwrap(),
        "<
  bf
  c'
  e'
  g'
>4"
    );

    let mut chord = Chord::new(&c_major(), Duration::new(1, 4));

    chord.insert(Pitch::new(
        PitchClass::new(DiatonicPitchClass::B, Accidental::Flat),
        4,
    ));

    assert_eq!(
        chord.to_lilypond().unwrap(),
        "<
  c'
  e'
  g'
  bf'
>4"
    );
}

#[test]
fn to_lilypond() {
    let pitches = c_major();

    let chord = Chord::new(&pitches, Duration::new(1, 4));

    assert_eq!(
        chord.to_lilypond().unwrap(),
        "<
  c'
  e'
  g'
>4"
    );
}
