use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccidentalDisplay {
    Cautionary,
    Forced,
}

impl fmt::Display for AccidentalDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cautionary => "?",
            Self::Forced => "!",
        };
        write!(f, "{s}")
    }
}
