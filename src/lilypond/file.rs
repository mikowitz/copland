use super::{executable, version, ToLilypond};
use crate::container::Containable;
use crate::error::Error;
use chrono;
use path_clean::PathClean;
use std::path::Path;
use std::{env, fs, io, process::Command};

const LILYPOND_TEMP_DIR: &str = "/tmp/copland";

#[must_use]
#[derive(Debug)]
pub struct File {
    source_path: Option<String>,
    output_path: Option<String>,
    content: Containable,
    // lilypond_options: Map<String, String>
}

type FileResult<'a> = Result<&'a File, Error>;

impl File {
    pub const fn new(content: Containable) -> Self {
        Self {
            content,
            source_path: None,
            output_path: None,
        }
    }

    /// Saves a `File`'s contents to the `copland` temp directory as a lilypond file (extension .ly)
    ///
    /// # Errors
    ///
    /// Will return an error if
    ///     * the copland temp directory does not exist and cannot be created
    ///     * the generated lilypond contents cannot be written to disk at the generated location
    ///
    /// # Panics
    ///
    /// Will panic if there is a problem rendering the `File`'s contents as lilypond
    pub fn save(&mut self) -> FileResult {
        Self::create_copland_tmp_dir()?;
        let file_contents = [file_prelude(), self.content.to_lilypond().unwrap()].join("\n\n");
        let filename = generate_filename();
        fs::write(&filename, file_contents)?;
        self.source_path = Some(filename);
        Ok(self)
    }

    /// Saves a `File`'s contents to the given filesystem location as a lilypond file (extension .ly)
    ///
    /// # Errors
    ///
    /// Will return an error if
    ///     * the copland temp directory does not exist and cannot be created
    ///     * the generated lilypond contents cannot be written to disk at the generated location
    ///
    /// # Panics
    ///
    /// Will panic if
    ///     * there is a problem rendering the `File`'s contents as lilypond
    ///     * the parent directory/directories of the provided path do not exist
    pub fn save_to(&mut self, path: &str) -> FileResult {
        let file_contents = [file_prelude(), self.content.to_lilypond().unwrap()].join("\n\n");
        let filename = Self::generate_filename_from(path).unwrap();
        fs::write(&filename, file_contents)?;
        self.source_path = Some(filename);
        Ok(self)
    }

    /// Compiles a saved `File` to PDF
    ///
    /// Will `save` the `File` if necessary.
    ///
    /// # Errors
    ///
    /// Will return an error if
    ///     * `save` fails
    ///     * an error occurs running the `lilypond` command
    ///
    /// # Panics
    ///
    /// Will panic if there is a problem reading the `File`'s source path
    pub fn compile(&mut self) -> FileResult {
        if self.source_path.is_none() {
            self.save()?;
        }

        let output_path = self.source_path.as_ref().unwrap().replace(".ly", "");
        Command::new(executable())
            .args(["-o", &output_path])
            .arg(self.source_path.as_ref().unwrap())
            .status()?;

        self.output_path = Some(format!("{output_path}.pdf"));
        Ok(self)
    }

    /// Displays a `File`'s contents, converted to PDF
    ///
    /// Will `save` and `compile` the `File` if necessary.
    ///
    /// # Errors
    ///
    /// Will return an error if
    ///     * either `save` or `compile` fails
    ///     * the system's `open` command fails to run
    ///
    /// # Panics
    ///
    /// Will panic if there is a problem reading the `File`'s source path or output path
    pub fn show(&mut self) -> FileResult {
        if self.source_path.is_none() {
            self.save()?;
        }
        if self.output_path.is_none() {
            self.compile()?;
        }
        Command::new("open")
            .arg(self.output_path.as_ref().unwrap())
            .status()?;
        Ok(self)
    }

    fn create_copland_tmp_dir() -> io::Result<()> {
        fs::create_dir_all(LILYPOND_TEMP_DIR)?;
        Ok(())
    }

    // https://stackoverflow.com/a/54817755
    fn generate_filename_from(path: &str) -> io::Result<String> {
        let path = Path::new(path);

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()?.join(path)
        }
        .clean();

        Ok(absolute_path.to_string_lossy().to_string())
    }
}

fn generate_filename() -> String {
    let now = chrono::Local::now();
    format!(
        "{LILYPOND_TEMP_DIR}/{}-{}.ly",
        now.format("%Y-%m-%d-%H-%M-%S"),
        now.timestamp_micros()
    )
}

fn file_prelude() -> String {
    format!("\\version \"{}\"\n\\language \"english\"", version())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::env;

    #[test]
    fn new() {
        let rest = Rest::new(Duration::new(1, 4)).unwrap();

        let container = Container::new(vec![rest.into()]);

        let file = File::new(container.into());

        assert_eq!(file.source_path, None);
        assert_eq!(file.output_path, None);
    }

    #[test]
    fn save() {
        let rest = Rest::new(Duration::new(1, 4)).unwrap();

        let container = Container::new(vec![rest.into()]);

        let mut file = File::new(container.into());
        file.save().unwrap();

        assert!(file.source_path.is_some());

        let Some(path) = file.source_path else { panic!() };
        assert!(path.contains("/tmp/copland"));
    }

    #[test]
    fn save_to() {
        let rest = Rest::new(Duration::new(1, 4)).unwrap();

        let container = Container::new(vec![rest.into()]);

        let mut file = File::new(container.into());
        file.save_to("test.ly").unwrap();
        //
        assert!(file.source_path.is_some());
        //
        let Some(path) = file.source_path else { panic!() };
        assert_eq!(
            path,
            env::current_dir()
                .unwrap()
                .join("test.ly")
                .to_string_lossy()
        );

        fs::remove_file(path).unwrap();
    }
}
