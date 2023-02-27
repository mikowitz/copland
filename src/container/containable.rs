use crate::chord::Chord;
use crate::container::Container;
use crate::error::Error;
use crate::lilypond::ToLilypond;
use crate::note::Note;
use crate::rest::Rest;
use crate::score::{RhythmicStaff, Score, Staff, StaffGroup, Voice};
use crate::spacer::Spacer;
use crate::tuplet::Tuplet;

macro_rules! containable {
    (
        $($type:ident) , *
    ) => {
        #[derive(Debug, Clone)]
        pub enum Containable {
            $(
            $type($type),
            )*
        }

        impl ToLilypond for Containable {
            fn to_lilypond(&self) -> Result<String, Error>
            {
                match self {
                    $(
                    Self::$type(x) => x.to_lilypond(),
                    )*
                }
            }
        }

        $(
        impl From<$type> for Containable {
            fn from(x: $type) -> Self {
                Self::$type(x)
            }
        }
        )*
    }
}

containable!(
    Chord,
    Note,
    Rest,
    Spacer,
    Container,
    Tuplet,
    Voice,
    RhythmicStaff,
    Staff,
    StaffGroup,
    Score
);

impl Containable {
    #[must_use]
    pub const fn is_leaf(&self) -> bool {
        matches!(
            self,
            Self::Chord(_) | Self::Note(_) | Self::Spacer(_) | Self::Rest(_)
        )
    }

    pub fn leaves(&self) -> Vec<Self> {
        if self.is_leaf() {
            self.contents()
        } else {
            self.contents()
                .iter()
                .flat_map(Self::leaves)
                .collect::<Vec<Self>>()
        }
    }

    #[must_use]
    pub fn contents(&self) -> Vec<Self> {
        match self {
            Self::Container(c) => c.contents().to_vec(),
            Self::Voice(c) => c.contents().to_vec(),
            Self::Staff(c) => c.contents().to_vec(),
            Self::StaffGroup(c) => c.contents().to_vec(),
            Self::Score(c) => c.contents().to_vec(),
            Self::Tuplet(c) => c.contents().to_vec(),
            Self::RhythmicStaff(c) => c.contents().to_vec(),
            Self::Chord(c) => vec![c.clone().into()],
            Self::Note(c) => vec![(*c).into()],
            Self::Rest(c) => vec![(*c).into()],
            Self::Spacer(c) => vec![(*c).into()],
        }
    }
}
