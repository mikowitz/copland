#[derive(Debug, PartialEq)]
pub enum StaffGroupContext {
    StaffGroup,
    ChoirStaff,
    GrandStaff,
    PianoStaff,
}

#[derive(Debug, PartialEq)]
pub enum StaffContext {
    Staff,
    RhythmicStaff,
    TabStaff,
    DrumStaff,
    VaticanaStaff,
    MensuralStaff,
}

#[derive(Debug, PartialEq)]
pub enum VoiceContext {
    Voice,
    VaticanaVoice,
    MensuralVoice,
    Lyrics,
    DrumVoice,
    FiguredBass,
    TabVoice,
    CueVoice,
    ChordNames,
}
