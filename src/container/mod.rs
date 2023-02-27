mod containable;
use crate::error::Error;
pub use containable::Containable;

use crate::lilypond::{format_contents, ToLilypond};

#[derive(Debug, Clone)]
pub struct Container {
    contents: Vec<Containable>,
}

impl Container {
    #[must_use]
    pub fn new(contents: Vec<Containable>) -> Self {
        Self { contents }
    }

    #[must_use]
    pub fn contents(&self) -> &[Containable] {
        &self.contents
    }
}

impl ToLilypond for Container {
    fn to_lilypond(&self) -> Result<String, Error> {
        Ok(format!("{{\n{}\n}}", format_contents(&self.contents)))
    }
}

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! container {
    (
        $($element:expr) , *
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($element.into());
            )*

            Container::new(v)
        }
    }

}
