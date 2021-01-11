extern crate clap;

use clap::{Arg, App};
use std::{fs, io};

fn f(path: String, arguments: &Arguments) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    if arguments.sort {
        entries.sort();
    }

    for entry in entries {
        if entry.as_path().is_dir() {
            println!("{}/", entry.display());
            f(entry.display().to_string(), arguments)?;
        } else {
            println!("{}", entry.display());
        }
    }
    Ok(())
}

struct Arguments {
    sort: bool,
}

impl Arguments {
    fn new(sort: bool) -> Self {
        Arguments {
            sort,
        }
    }
}

fn main() {
    // Command line parameters.
    let matches = App::new("find")
        .version("0.1")
        .about("rust-version of the find-command")
        .author("Claus Guttesen")
        .arg(Arg::with_name("file hierarchy")
            .help("search-path to traverse")
            .required(false)
            .takes_value(true)
            .short("f")
            .multiple(true)
        )
        .arg(Arg::with_name("sort")
            .help("alphabetical sort")
            .required(false)
            .takes_value(false)
            .short("s")
            .long("sort")
            .multiple(false)
        )
        .get_matches();

    let sort: bool = matches.is_present("sort");
    let arguments = Arguments::new(sort);

    if matches.is_present("file hierarchy") {
        let paths: Vec<_> = matches.values_of("file hierarchy").unwrap().collect();
        for path in paths {
            let _r = f(path.to_string(), &arguments);
        }
    } else {
        println!("{}", matches.usage());
    }
}
