use std::path::Path;

// TODO Handle all the unwraps here
pub fn copy_static_files<D: AsRef<Path>>(files: D, dir: D) {
    let files = files.as_ref();
    let dir = dir.as_ref();

    let files = files.read_dir().unwrap();

    for f in files {
        let f = f.unwrap();

        let ft = f.file_type().unwrap();

        if ft.is_dir() {
            std::fs::create_dir(dir.join(f.path())).unwrap();
            copy_static_files(f.path(), dir.join(f.path()));
        } else {
            std::fs::copy(f.path(), dir.join(f.file_name())).unwrap();
        }
    }
}
