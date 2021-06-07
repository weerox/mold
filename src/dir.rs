use std::path::{Path, PathBuf};

pub const TEMPLATES: &str = "templates/";
pub const MODULES: &str = "modules/";
pub const CONTENT: &str = "content/";
pub const STATIC: &str = "static/";

pub fn directories() -> Vec<PathBuf> {
    let dirs = vec![
        Path::new(TEMPLATES),
        Path::new(MODULES),
        Path::new(CONTENT),
        Path::new(STATIC),
    ];

    dirs.iter().map(|p| p.to_path_buf()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_count() {
        assert_eq!(4, directories().len());
    }
}
