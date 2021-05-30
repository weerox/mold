use std::path::{Path, PathBuf};

pub fn directories() -> Vec<PathBuf> {
    let dirs = vec![
        Path::new("templates/"),
        Path::new("modules/"),
        Path::new("content/"),
        Path::new("static/"),
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
