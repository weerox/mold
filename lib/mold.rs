use std::path::PathBuf;

pub struct Mold {
    paths: Paths,
}

impl Mold {
    pub fn new() -> Mold {
        Mold {
            ..Default::default()
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
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
            modules: Vec::new(),
            content: Vec::new(),
            statics: Vec::new(),
        }
    }
}
