use super::{ScoreObject, StaffContext, StaffGroupContext, VoiceContext};

impl ScoreObject {
    pub fn staff_group_context(&self) -> Option<&StaffGroupContext> {
        match self {
            Self::StaffGroup { context, .. } => Some(context),
            _ => None,
        }
    }

    pub fn set_staff_group_context(&mut self, new_context: StaffGroupContext) {
        if let Self::StaffGroup { context, .. } = self {
            *context = new_context
        }
    }

    pub fn staff_context(&self) -> Option<&StaffContext> {
        match self {
            Self::Staff { context, .. } => Some(context),
            _ => None,
        }
    }

    pub fn set_staff_context(&mut self, new_context: StaffContext) {
        if let Self::Staff { context, .. } = self {
            *context = new_context
        }
    }

    pub fn voice_context(&self) -> Option<&VoiceContext> {
        match self {
            Self::Voice { context, .. } => Some(context),
            _ => None,
        }
    }

    pub fn set_voice_context(&mut self, new_context: VoiceContext) {
        if let Self::Voice { context, .. } = self {
            *context = new_context
        }
    }
}
