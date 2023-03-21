use super::{
    chord, container,
    contexts::{StaffContext, StaffGroupContext, VoiceContext},
    note, rest, staff, staff_group, tuplet, voice,
};
use crate::{
    duration::Duration,
    multiplier::Multiplier,
    pitch::{Accidental::*, DiatonicPitchClass::*, Pitch, PitchClass},
    score_object::ScoreObject,
};

#[test]
fn new_note() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4));
    assert!(n.is_ok());
}

#[test]
fn unprintable_note() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 3));
    assert!(n.is_err());
}

#[test]
fn new_rest() {
    let r = rest(Duration::new(2, 1));
    assert!(r.is_ok());
}

#[test]
fn unprintable_rest() {
    let r = rest(Duration::new(17, 1));
    assert!(r.is_err());
}

#[test]
fn new_container() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();
    let c = container(vec![n, r]);

    assert!(c.is_ok());
}

#[test]
fn new_chord() {
    let pitches: Vec<Pitch> = [C, E, G]
        .iter()
        .map(|dpc| Pitch::new(PitchClass::new(*dpc, Natural), 4))
        .collect();

    let ch = chord(pitches, Duration::new(1, 4));

    assert!(ch.is_ok());
}

#[test]
fn new_tuplet() {
    let notes: Vec<ScoreObject> = [C, E, G]
        .iter()
        .map(|dpc| {
            note(
                Pitch::new(PitchClass::new(*dpc, Natural), 4),
                Duration::new(1, 4),
            )
            .unwrap()
        })
        .collect();

    let tup = tuplet(Multiplier::new(2, 3), notes);
    assert!(tup.is_ok());
}

#[test]
fn is_leaf() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();

    assert!(n.is_leaf());
    assert!(r.is_leaf());

    let c = container(vec![n, r]).unwrap();

    assert!(!c.is_leaf());
}

#[test]
fn is_container() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();

    assert!(!n.is_container());
    assert!(!r.is_container());

    let c = container(vec![n, r]).unwrap();

    assert!(c.is_container());
}

#[test]
fn set_simultaneous() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();
    let mut c = container(vec![n, r]).unwrap();

    assert!(!c.is_simultaneous());

    c.set_is_simultaneous(true);

    assert!(c.is_simultaneous());
}

#[test]
fn set_name() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();
    let mut v = voice(vec![n, r]).unwrap();

    assert!(v.name().is_none());

    v.set_name(Some(String::from("Voice 1")));

    assert_eq!(v.name(), &Some("Voice 1".to_string()));
}

#[test]
fn set_context() {
    let n = note(Pitch::new(PitchClass::new(D, Flat), 4), Duration::new(1, 4)).unwrap();
    let r = rest(Duration::new(2, 1)).unwrap();
    let mut v = voice(vec![n, r]).unwrap();

    assert_eq!(v.voice_context(), Some(&VoiceContext::Voice));
    v.set_voice_context(VoiceContext::VaticanaVoice);
    assert_eq!(v.voice_context(), Some(&VoiceContext::VaticanaVoice));

    let mut s = staff(vec![v]).unwrap();
    assert_eq!(s.staff_context(), Some(&StaffContext::Staff));
    s.set_staff_context(StaffContext::RhythmicStaff);
    assert_eq!(s.staff_context(), Some(&StaffContext::RhythmicStaff));
    assert!(s.voice_context().is_none());

    let mut sg = staff_group(vec![s]).unwrap();
    assert_eq!(
        sg.staff_group_context(),
        Some(&StaffGroupContext::StaffGroup)
    );
    sg.set_staff_group_context(StaffGroupContext::PianoStaff);
    assert_eq!(
        sg.staff_group_context(),
        Some(&StaffGroupContext::PianoStaff)
    );
    assert!(sg.staff_context().is_none());
}
