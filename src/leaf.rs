use crate::chord::Chord;
use crate::duration::Duration;
use crate::note::Note;
use crate::rest::Rest;
use crate::spacer::Spacer;
use std::fmt::Debug;

pub trait Leaf: Debug {
    fn written_duration(&self) -> Duration;

    /// Converts the given `Leaf` to a `Note`
    fn to_note(&self) -> Note;

    /// Converts the given `Leaf` to a `Rest`
    fn to_rest(&self) -> Rest;

    /// Converts the given `Leaf` to a `Chord`
    fn to_chord(&self) -> Chord;

    /// Converts the given `Leaf` to a `Spacer`
    fn to_spacer(&self) -> Spacer;
}
