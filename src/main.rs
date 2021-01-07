extern crate clap;

use clap::{Arg, App};
use std::{fs, io};

fn f(path: String) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        if entry.as_path().is_dir() {
            println!("{}/", entry.display());
            f(entry.display().to_string())?;
        } else {
            println!("{}", entry.display());
        }
    }
    Ok(())
}

struct Arguments {
    path: String,
}

impl Arguments {
    fn new(path: String) -> Self {
        Arguments {
            path,
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
            .multiple(false)
            .default_value(".")
        )
        .get_matches();

    let path = matches.value_of("file hierarchy").unwrap();

    let arguments = Arguments::new(path.to_string());

    println!("find-command in rust");

    let _r = f(arguments.path);
}
