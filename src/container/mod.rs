mod containable;
pub use containable::Containable;

use crate::to_lilypond::{format_contents, ToLilypond};

#[derive(Debug)]
pub struct Container {
    contents: Vec<Containable>,
}

impl Container {
    #[must_use]
    pub fn new(contents: Vec<Containable>) -> Self {
        Self { contents }
    }
}

impl ToLilypond for Container {
    fn to_lilypond(&self) -> Result<String, crate::error::Error>
    where
        Self: std::fmt::Debug,
    {
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
