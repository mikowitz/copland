use super::{Direction, Position};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachable {
    Tie,
}

impl Attachable {
    pub fn defaults(self) -> (Option<Direction>, Option<Position>, i32) {
        match self {
            Self::Tie => (None, None, 0),
        }
    }
}
