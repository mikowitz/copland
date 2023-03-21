use crate::{duration::Duration, error::Error, multiplier::Multiplier, pitch::Pitch};

mod contexts;
pub use contexts::{StaffContext, StaffGroupContext, VoiceContext};

mod has_context;
mod indexed;
mod is_simultaneous;
mod named;

#[derive(Debug, PartialEq)]
pub enum ScoreObject {
    Note {
        written_pitch: Pitch,
        written_duration: Duration,
    },
    Rest {
        written_duration: Duration,
    },
    Spacer {
        written_duration: Duration,
    },
    Chord {
        written_pitches: Box<Vec<Pitch>>,
        written_duration: Duration,
    },
    Tuplet {
        multiplier: Multiplier,
        contents: Box<Vec<ScoreObject>>,
    },
    Container {
        contents: Box<Vec<ScoreObject>>,
        is_simultaneous: bool,
    },
    Voice {
        contents: Box<Vec<ScoreObject>>,
        is_simultaneous: bool,
        name: Option<String>,
        context: VoiceContext,
    },
    Staff {
        contents: Box<Vec<ScoreObject>>,
        is_simultaneous: bool,
        name: Option<String>,
        context: StaffContext,
    },
    StaffGroup {
        contents: Box<Vec<ScoreObject>>,
        is_simultaneous: bool,
        name: Option<String>,
        context: StaffGroupContext,
    },
    Score {
        contents: Box<Vec<ScoreObject>>,
        is_simultaneous: bool,
        name: Option<String>,
    },
}

impl ScoreObject {
    pub fn is_leaf(&self) -> bool {
        matches!(
            self,
            Self::Note { .. } | Self::Chord { .. } | Self::Rest { .. } | Self::Spacer { .. }
        )
    }

    pub fn is_container(&self) -> bool {
        !self.is_leaf()
    }
}

pub fn note(written_pitch: Pitch, written_duration: Duration) -> Result<ScoreObject, Error> {
    if written_duration.is_printable() {
        Ok(ScoreObject::Note {
            written_pitch,
            written_duration,
        })
    } else {
        Err(Error::UnprintableDuration(written_duration))
    }
}

pub fn rest(written_duration: Duration) -> Result<ScoreObject, Error> {
    if written_duration.is_printable() {
        Ok(ScoreObject::Rest { written_duration })
    } else {
        Err(Error::UnprintableDuration(written_duration))
    }
}

pub fn chord(pitches: Vec<Pitch>, written_duration: Duration) -> Result<ScoreObject, Error> {
    if written_duration.is_printable() {
        let written_pitches = Box::new(pitches);
        Ok(ScoreObject::Chord {
            written_pitches,
            written_duration,
        })
    } else {
        Err(Error::UnprintableDuration(written_duration))
    }
}

pub fn tuplet(multiplier: Multiplier, contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::Tuplet {
        multiplier,
        contents,
    })
}

pub fn container(contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::Container {
        contents,
        is_simultaneous: false,
    })
}

pub fn voice(contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::Voice {
        contents,
        is_simultaneous: false,
        name: None,
        context: VoiceContext::Voice,
    })
}

pub fn staff(contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::Staff {
        contents,
        is_simultaneous: false,
        name: None,
        context: StaffContext::Staff,
    })
}

pub fn staff_group(contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::StaffGroup {
        contents,
        is_simultaneous: false,
        name: None,
        context: StaffGroupContext::StaffGroup,
    })
}

pub fn score(contents: Vec<ScoreObject>) -> Result<ScoreObject, Error> {
    let contents = Box::new(contents);
    Ok(ScoreObject::Score {
        contents,
        is_simultaneous: false,
        name: None,
    })
}

#[cfg(test)]
mod tests;
