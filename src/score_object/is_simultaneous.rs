use super::ScoreObject;

impl ScoreObject {
    pub fn is_simultaneous(&self) -> bool {
        match self {
            Self::Container {
                is_simultaneous, ..
            } => *is_simultaneous,
            Self::Voice {
                is_simultaneous, ..
            } => *is_simultaneous,
            Self::Staff {
                is_simultaneous, ..
            } => *is_simultaneous,
            Self::StaffGroup {
                is_simultaneous, ..
            } => *is_simultaneous,
            Self::Score {
                is_simultaneous, ..
            } => *is_simultaneous,
            _ => false,
        }
    }

    pub fn set_is_simultaneous(&mut self, new_is_simultaneous: bool) {
        match self {
            Self::Container {
                is_simultaneous, ..
            } => *is_simultaneous = new_is_simultaneous,
            Self::Voice {
                is_simultaneous, ..
            } => *is_simultaneous = new_is_simultaneous,
            Self::Staff {
                is_simultaneous, ..
            } => *is_simultaneous = new_is_simultaneous,
            Self::StaffGroup {
                is_simultaneous, ..
            } => *is_simultaneous = new_is_simultaneous,
            Self::Score {
                is_simultaneous, ..
            } => *is_simultaneous = new_is_simultaneous,
            _ => (),
        }
    }
}
