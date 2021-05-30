use crate::dir;

use std::fs::create_dir;
use std::path::Path;

use clap::ArgMatches;

pub fn exec(args: &ArgMatches) {
    let name = args.value_of("name").unwrap();

    create_dir(name).unwrap();

    let dirs = dir::directories();
    let dirs = dirs.iter().map(|d| Path::new(name).join(d));

    for dir in dirs {
        create_dir(dir).unwrap();
    }
}
