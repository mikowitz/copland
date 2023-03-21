use std::ops::{Index, IndexMut};

use super::ScoreObject;

impl ScoreObject {
    pub fn contents(&self) -> Option<&Vec<ScoreObject>> {
        match self {
            Self::Tuplet { contents, .. } => Some(contents),
            Self::Container { contents, .. } => Some(contents),
            Self::Voice { contents, .. } => Some(contents),
            Self::Staff { contents, .. } => Some(contents),
            Self::StaffGroup { contents, .. } => Some(contents),
            Self::Score { contents, .. } => Some(contents),
            _ => None,
        }
    }

    pub fn contents_mut(&mut self) -> Option<&mut Vec<ScoreObject>> {
        match self {
            Self::Tuplet {
                ref mut contents, ..
            } => Some(contents),
            Self::Container {
                ref mut contents, ..
            } => Some(contents),
            Self::Voice {
                ref mut contents, ..
            } => Some(contents),
            Self::Staff {
                ref mut contents, ..
            } => Some(contents),
            Self::StaffGroup {
                ref mut contents, ..
            } => Some(contents),
            Self::Score {
                ref mut contents, ..
            } => Some(contents),
            _ => None,
        }
    }
}

impl Index<usize> for ScoreObject {
    type Output = ScoreObject;

    fn index(&self, index: usize) -> &Self::Output {
        match self.contents() {
            Some(contents) => &contents[index],
            None => panic!(),
        }
    }
}

impl IndexMut<usize> for ScoreObject {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.contents_mut() {
            Some(contents) => &mut contents[index],
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::duration::Duration;
    use crate::multiplier::Multiplier;
    use crate::pitch::{Accidental::*, DiatonicPitchClass::*, Pitch, PitchClass};
    use crate::score_object::{note, tuplet, ScoreObject};

    fn tuplet_fixture() -> ScoreObject {
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

        tuplet(Multiplier::new(2, 3), notes).unwrap()
    }

    #[test]
    fn indexed() {
        assert_eq!(
            tuplet_fixture()[0],
            note(
                Pitch::new(PitchClass::new(C, Natural), 4),
                Duration::new(1, 4)
            )
            .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn indexed_out_of_bounds() {
        let _fourth_note = &tuplet_fixture()[3];
    }

    #[test]
    fn indexed_mut() {
        let mut t = tuplet_fixture();
        t[0] = note(
            Pitch::new(PitchClass::new(C, Sharp), 4),
            Duration::new(1, 4),
        )
        .unwrap();
        assert_eq!(
            t[0],
            note(
                Pitch::new(PitchClass::new(C, Sharp), 4),
                Duration::new(1, 4)
            )
            .unwrap()
        );
    }
}
