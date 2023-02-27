#![feature(const_option)]
#![feature(const_result_drop)]

#[cfg(test)]
#[macro_use]
pub mod regression_data_streamer;

pub mod chord;
pub mod container;
pub mod duration;
pub mod error;
pub mod interval;
pub mod leaf;
pub mod lilypond;
pub mod note;
pub mod notehead;
pub mod pitch;
pub mod rest;
pub mod score;
pub mod spacer;
pub mod tuplet;

pub mod prelude {
    pub use crate::chord::Chord;
    pub use crate::container;
    pub use crate::container::{Containable, Container};
    pub use crate::duration::Duration;
    pub use crate::lilypond;
    pub use crate::lilypond::{File, ToLilypond};
    pub use crate::note::Note;
    pub use crate::pitch::*;
    pub use crate::rest::Rest;
    pub use crate::score::{RhythmicStaff, Score, Staff, StaffGroup, Voice};
    pub use crate::spacer::Spacer;
    pub use crate::tuplet::{Multiplier, Tuplet};
}
