use std::path::PathBuf;

use tempfile::{TempDir, Builder};

use crate::templates;
use crate::statics;

pub struct Mold {
    paths: Paths,
}

impl Mold {
    pub fn new<D: Into<PathBuf>>(output: D) -> Mold {
        let mut m = Mold {
            ..Default::default()
        };

        m.paths.output = output.into();

        m
    }

    pub fn add_templates_dir<D: Into<PathBuf>>(&mut self, dir: D) {
        self.paths.templates.push(dir.into());
    }

    pub fn add_modules_dir<D: Into<PathBuf>>(&mut self, dir: D) {
        self.paths.modules.push(dir.into());
    }

    pub fn add_content_dir<D: Into<PathBuf>>(&mut self, dir: D) {
        self.paths.content.push(dir.into());
    }

    pub fn add_static_dir<D: Into<PathBuf>>(&mut self, dir: D) {
        self.paths.statics.push(dir.into());
    }

    pub fn build(&self) {
        templates::flatten_templates(&self.paths.templates, &self.paths.build);

        // Copy static files to output directory
        for dir in &self.paths.statics {
            statics::copy_static_files(dir, &self.paths.output);
        }
    }
}

impl Default for Mold {
    fn default() -> Self {
        Self {
            paths: Default::default(),
        }
    }
}

struct Paths {
    templates: Vec<PathBuf>,
    modules: Vec<PathBuf>,
    content: Vec<PathBuf>,
    // NOTE 'static' is a reserved keyword
    statics: Vec<PathBuf>,
    // Directory of built site
    output: PathBuf,
    // Directory to store temporary build files
    build: TempDir,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
            modules: Vec::new(),
            content: Vec::new(),
            statics: Vec::new(),
            output: PathBuf::new(),
            build: Builder::new().prefix("mold").tempdir().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_output() {
        let m = Mold::new("out");

        assert_eq!(m.paths.output, PathBuf::from("out"));
    }

    #[test]
    fn add_single_template_dir() {
        let mut m = Mold::new("out/");

        m.add_templates_dir("test/");

        assert_eq!(1, m.paths.templates.len());
        assert_eq!(m.paths.templates.get(0), Some(&PathBuf::from("test/")));
    }

    #[test]
    fn add_single_modules_dir() {
        let mut m = Mold::new("out/");

        m.add_modules_dir("test/");

        assert_eq!(1, m.paths.modules.len());
        assert_eq!(m.paths.modules.get(0), Some(&PathBuf::from("test/")));
    }

    #[test]
    fn add_single_content_dir() {
        let mut m = Mold::new("out/");

        m.add_content_dir("test/");

        assert_eq!(1, m.paths.content.len());
        assert_eq!(m.paths.content.get(0), Some(&PathBuf::from("test/")));
    }

    #[test]
    fn add_single_static_dir() {
        let mut m = Mold::new("out/");

        m.add_static_dir("test/");

        assert_eq!(1, m.paths.statics.len());
        assert_eq!(m.paths.statics.get(0), Some(&PathBuf::from("test/")));
    }
}
