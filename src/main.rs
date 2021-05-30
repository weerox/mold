mod cmd;
mod dir;

use clap::{App, SubCommand, Arg};
use clap::{crate_name, crate_version};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        ("init", Some(sub_matches)) => cmd::init::exec(sub_matches),
        ("new", Some(sub_matches)) => cmd::new::exec(sub_matches),
        _ => {
            cli().print_help().unwrap();
            return;
        },
    };
}

fn cli() -> App<'static, 'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("init"))
        .subcommand(SubCommand::with_name("new")
            .arg(Arg::with_name("name").required(true)));
    app
}
