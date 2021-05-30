use crate::dir;

use std::fs::create_dir;

use clap::ArgMatches;

pub fn exec(_args: &ArgMatches) {
    let dirs = dir::directories();

    for dir in dirs {
        create_dir(dir).unwrap();
    }
}
