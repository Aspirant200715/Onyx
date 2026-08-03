use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Handles reading static files from a configured root directory.
pub struct StaticFiles {
    root: PathBuf,
}

impl StaticFiles {
    /// Creates a new static file service.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the configured root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Reads a file relative to the root directory.
    pub fn read(&self, path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
        fs::read(self.root.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn creates_static_service() {
        let files = StaticFiles::new("public");

        assert_eq!(files.root(), Path::new("public"),);
    }

    #[test]
    fn reads_existing_file() {
        fs::create_dir_all("test_public").unwrap();

        fs::write("test_public/index.html", "Hello Ember").unwrap();

        let files = StaticFiles::new("test_public");

        let bytes = files.read("index.html").unwrap();

        assert_eq!(String::from_utf8(bytes).unwrap(), "Hello Ember",);

        fs::remove_dir_all("test_public").unwrap();
    }

    #[test]
    fn returns_error_for_missing_file() {
        let files = StaticFiles::new("public");

        assert!(files.read("missing.html").is_err());
    }
}
