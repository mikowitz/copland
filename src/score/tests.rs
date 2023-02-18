use crate::prelude::*;

fn note(dpc: DiatonicPitchClass) -> Note {
    Note::new(
        Pitch::new(PitchClass::new(dpc, Accidental::Natural), 4),
        Duration::new(1, 4),
    )
    .unwrap()
}

#[test]
fn to_lilypond() {
    let mut staff = Staff::new(vec![
        note(DiatonicPitchClass::C).into(),
        note(DiatonicPitchClass::D).into(),
        note(DiatonicPitchClass::E).into(),
    ]);
    staff.set_name("Staff One");

    let mut staff2 = Staff::new(vec![
        note(DiatonicPitchClass::F).into(),
        note(DiatonicPitchClass::G).into(),
        note(DiatonicPitchClass::A).into(),
    ]);
    staff2.set_name("Staff Two");

    let mut staff_group = StaffGroup::new(vec![staff.into(), staff2.into()]);
    staff_group.set_name("Staff Group");

    let mut score = Score::new(vec![staff_group.into()]);

    assert_eq!(
        score.to_lilypond().unwrap(),
        r#"
\new Score <<
  \context StaffGroup = "Staff Group" <<
    \context Staff = "Staff One" {
      c'4
      d'4
      e'4
    }
    \context Staff = "Staff Two" {
      f'4
      g'4
      a'4
    }
  >>
>>
    "#
        .trim()
    );

    score.set_name("My Score");

    assert_eq!(
        score.to_lilypond().unwrap(),
        r#"
\context Score = "My Score" <<
  \context StaffGroup = "Staff Group" <<
    \context Staff = "Staff One" {
      c'4
      d'4
      e'4
    }
    \context Staff = "Staff Two" {
      f'4
      g'4
      a'4
    }
  >>
>>
    "#
        .trim()
    );
}
