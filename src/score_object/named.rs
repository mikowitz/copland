use super::ScoreObject;

impl ScoreObject {
    pub fn name(&self) -> &Option<String> {
        match self {
            Self::Voice { name, .. } => name,
            Self::Staff { name, .. } => name,
            Self::StaffGroup { name, .. } => name,
            Self::Score { name, .. } => name,
            _ => &None,
        }
    }

    pub fn set_name(&mut self, new_name: Option<String>) {
        match self {
            Self::Voice { name, .. } => *name = new_name,
            Self::Staff { name, .. } => *name = new_name,
            Self::StaffGroup { name, .. } => *name = new_name,
            Self::Score { name, .. } => *name = new_name,
            _ => (),
        }
    }
}
