use crate::chord::Chord;
use crate::container::Container;
use crate::note::Note;
use crate::rest::Rest;
use crate::score::{Score, Staff, StaffGroup, Voice};
use crate::spacer::Spacer;
use crate::to_lilypond::ToLilypond;
use crate::tuplet::Tuplet;

macro_rules! containable {
    (
        $($type:ident) , *
    ) => {
        #[derive(Debug)]
        pub enum Containable {
            $(
            $type($type),
            )*
        }

        impl ToLilypond for Containable {
            fn to_lilypond(&self) -> Result<String, crate::error::Error>
            where
                Self: std::fmt::Debug,
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

containable!(Chord, Note, Rest, Spacer, Container, Tuplet, Voice, Staff, StaffGroup, Score);
