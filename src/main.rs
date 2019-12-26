mod bf;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let args = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
            .arg(Arg::with_name("path")
                .help("BrainFuck source file")
                .required(true)
            )
            .arg(Arg::with_name("dump")
                .help("memory dump flag")
                .short("d")
                .long("dump")
            ).get_matches();
            

    let flg = bf::Flg {
        dump: args.is_present("dump")
    };

    if let Some(path) = args.value_of("path") {
        bf::machine(path.to_string(), flg);
    }
}
