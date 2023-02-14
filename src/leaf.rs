use crate::chord::Chord;
use crate::duration::Duration;
use crate::note::Note;
use crate::rest::Rest;
use crate::spacer::Spacer;
use std::fmt::Debug;

pub trait Leaf: Debug {
    type Note = Note;
    type Rest = Rest;
    type Chord = Chord;
    type Spacer = Spacer;
    type Duration = Duration;

    fn written_duration(&self) -> Self::Duration;

    /// Converts the given `Leaf` to a `Note`
    fn to_note(&self) -> Self::Note;

    /// Converts the given `Leaf` to a `Rest`
    fn to_rest(&self) -> Self::Rest;

    /// Converts the given `Leaf` to a `Chord`
    fn to_chord(&self) -> Self::Chord;

    /// Converts the given `Leaf` to a `Spacer`
    fn to_spacer(&self) -> Self::Spacer;
}
