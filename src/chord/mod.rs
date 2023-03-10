use itertools::Itertools;

use crate::duration::Duration;
use crate::error::Error;
use crate::interval::Interval;
use crate::leaf::Leaf;
use crate::lilypond::{indent, prepare_attachments, ToLilypond};
use crate::notehead::Notehead;
use crate::pitch::Pitch;
use crate::prelude::Attachment;

type NoteheadList = Vec<Notehead>;

#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    noteheads: NoteheadList,
    written_duration: Duration,
    attachments: Vec<Attachment>,
}

impl Chord {
    /// Returns a new `Chord` from the given list of `Pitch`es and `Duration`
    ///
    /// # Errors
    ///
    /// Returns an `UnprintableDuration` error if the given `Duration` cannot be rendered by a
    /// single notehead in a score.
    pub fn new(pitches: &[Pitch], written_duration: Duration) -> Result<Self, Error> {
        if written_duration.is_printable() {
            let mut noteheads = Self::pitches_to_notehead_list(pitches);
            noteheads.sort();
            Ok(Self {
                noteheads,
                written_duration,
                attachments: vec![],
            })
        } else {
            Err(Error::UnprintableDuration(written_duration))
        }
    }

    #[must_use]
    pub fn written_pitches(&self) -> Vec<Pitch> {
        self.noteheads
            .iter()
            .map(|&n| n.written_pitch())
            .collect::<Vec<Pitch>>()
    }

    pub fn insert(&mut self, pitch: Pitch) {
        self.noteheads.insert(0, Notehead::new(pitch));
        self.noteheads.sort();
    }

    pub fn transpose(&mut self, interval: Interval) {
        self.noteheads
            .iter_mut()
            .for_each(|n| n.transpose(interval));
    }

    #[must_use]
    fn pitches_to_notehead_list(pitches: &[Pitch]) -> NoteheadList {
        pitches
            .iter()
            .map(|&p| Notehead::new(p))
            .collect::<NoteheadList>()
    }

    pub fn attach(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }
}

impl Leaf for Chord {
    fn written_duration(&self) -> crate::duration::Duration {
        self.written_duration
    }

    fn to_note(&self) -> crate::note::Note {
        crate::note::Note::new(self.written_pitches()[0], self.written_duration).unwrap()
    }

    fn to_rest(&self) -> crate::rest::Rest {
        crate::rest::Rest::new(self.written_duration).unwrap()
    }

    fn to_spacer(&self) -> crate::spacer::Spacer {
        crate::spacer::Spacer::new(self.written_duration).unwrap()
    }

    fn to_chord(&self) -> Self {
        self.clone()
    }
}

impl ToLilypond for Chord {
    fn to_lilypond(&self) -> Result<String, Error> {
        let (b, a) = prepare_attachments(&self.attachments);
        Ok(format!(
            "{}\n<\n{}\n>{}\n{}",
            b.trim(),
            noteheads_to_lily(&self.noteheads),
            self.written_duration.to_lilypond().unwrap(),
            indent(a.trim()),
        )
        .trim()
        .to_string())
    }
}

fn noteheads_to_lily(noteheads: &NoteheadList) -> String {
    noteheads
        .iter()
        .map(|n| format!("  {}", n.to_lilypond().unwrap()))
        .join("\n")
}

#[cfg(test)]
mod tests;
