use super::{Direction, Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachable {
    Tie,
}

pub type Components = (Vec<String>, Vec<String>);

impl Attachable {
    pub fn defaults(self) -> (Option<Direction>, Option<Position>, i32) {
        match self {
            Self::Tie => (None, None, 0),
        }
    }

    pub fn components(self) -> Components {
        match self {
            Self::Tie => tie_components(),
        }
    }
}

fn tie_components() -> Components {
    (vec![], vec!["~".to_string()])
}
