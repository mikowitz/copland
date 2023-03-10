use super::{Direction, Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArpeggioStyle {
    Normal,
    ArrowUp,
    ArrowDown,
    Bracket,
    Parenthesis,
    ParenthesisDashed,
}

impl ArpeggioStyle {
    fn as_component(self) -> String {
        format!("\\arpeggio{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachable {
    Arpeggio(ArpeggioStyle),
    Tie,
}

pub type Components = (Vec<String>, Vec<String>);

impl Attachable {
    #[must_use]
    pub const fn defaults(self) -> (Option<Direction>, Option<Position>, i32) {
        match self {
            Self::Tie => (None, None, 0),
            Self::Arpeggio(_) => (None, None, 0),
        }
    }

    #[must_use]
    pub fn components(self) -> Components {
        match self {
            Self::Tie => tie_components(),
            Self::Arpeggio(style) => arpeggio_components(style),
        }
    }

    #[must_use]
    pub fn has_direction(self) -> bool {
        if matches!(self, Self::Tie) {
            true
        } else {
            false
        }
    }
}

fn tie_components() -> Components {
    (vec![], vec!["~".to_string()])
}

fn arpeggio_components(style: ArpeggioStyle) -> Components {
    (vec![style.as_component()], vec!["\\arpeggio".to_string()])
}
