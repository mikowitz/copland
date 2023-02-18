use super::*;
use crate::chord::Chord;
use crate::duration::*;
use crate::note::Note;
use crate::pitch::*;
use crate::rest::Rest;
use crate::spacer::Spacer;

fn note() -> Note {
    Note::new(
        Pitch::new(PitchClass::new(DiatonicPitchClass::E, Accidental::Flat), 2),
        Duration::new(1, 8),
    )
    .unwrap()
}

fn rest() -> Rest {
    Rest::new(Duration::new(3, 4)).unwrap()
}

fn spacer() -> Spacer {
    Spacer::new(Duration::new(3, 4)).unwrap()
}

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

fn chord() -> Chord {
    Chord::new(&c_major(), Duration::new(1, 4)).unwrap()
}

#[test]
fn new() {
    let container = Container::new(vec![
        note().into(),
        rest().into(),
        chord().into(),
        spacer().into(),
    ]);

    assert_eq!(
        container.to_lilypond().unwrap(),
        "{
  ef,8
  r2.
  <
    c'
    e'
    g'
  >4
  s2.
}"
    );

    let c2 = Container::new(vec![container.into()]);
    assert_eq!(
        c2.to_lilypond().unwrap(),
        "{
  {
    ef,8
    r2.
    <
      c'
      e'
      g'
    >4
    s2.
  }
}"
    );
}
