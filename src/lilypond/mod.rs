use regex::Regex;

mod to_lilypond;
pub use to_lilypond::*;
mod file;
pub use file::File;

use std::{process::Command, str};

/// Finds the installed `lilypond` executable, if one exists.
///
/// # Panics
///
/// Will panic if a string slice cannot be constructed from the `which` command's output
#[must_use]
pub fn executable() -> String {
    let lily_exec_bytes = Command::new("which")
        .arg("lilypond")
        .output()
        .unwrap()
        .stdout;

    str::from_utf8(&lily_exec_bytes).unwrap().trim().to_string()
}

/// Finds the version for a discovered `lilypond` executable
///
/// # Panics
///
/// Will panic if the version regex fails to compile
#[must_use]
pub fn version() -> String {
    Command::new(executable())
        .arg("--version")
        .output()
        .map_or(String::from("2.24.0"), |version| {
            let s = String::from_utf8_lossy(&version.stdout);
            let str_version = &Regex::new(r"[\d\.]+").unwrap().captures(&s).unwrap()[0];
            String::from(str_version)
        })
}
