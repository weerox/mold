use std::process::Command;

use tempfile::Builder;

#[test]
fn correct_dir_setup() {
    let temp_dir = Builder::new().prefix("mold").tempdir().expect("Could not create temporary directory");

    let bin = env!("CARGO_BIN_EXE_mold");

    let output = Command::new(bin)
        .arg("init")
        .current_dir(&temp_dir)
        .output().expect("Failed to run binary");


    assert!(output.status.success());

    let iter = temp_dir.path().read_dir().expect("Could not read temporary directory");
    assert_eq!(iter.count(), 4);
}
