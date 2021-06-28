use std::process::Command;

use tempdir::TempDir;

#[test]
fn correct_dir_setup() {
    let temp_dir = TempDir::new("mold").expect("Could not create temporary directory");

    let bin = env!("CARGO_BIN_EXE_mold");

    let output = Command::new(bin)
        .arg("init")
        .current_dir(&temp_dir)
        .output().expect("Failed to run binary");


    assert!(output.status.success());

    let iter = temp_dir.path().read_dir().expect("Could not read temporary directory");
    assert_eq!(iter.count(), 4);
}
