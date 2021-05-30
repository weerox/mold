use clap::App;
use clap::{crate_name, crate_version};

fn main() {
    let matches = cli().get_matches();
}

fn cli() -> App<'static, 'static> {
    let app = App::new(crate_name!())
        .version(crate_version!());
    app
}
