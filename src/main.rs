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

struct Condition(String, i32);

struct Arguments {
    sort: bool,
    bmin: Condition,
}

impl Arguments {
    fn new(
        sort: Option<bool>,
        bmin: Option<Condition>,
    ) -> Self {
        Arguments {
            sort: sort.unwrap(),
            bmin: bmin.unwrap(),
        }
    }
}

fn split_parameter(p: &str) -> Condition {
    if p.starts_with("+") || p.starts_with("-") {
        let (first, last) = p.split_at(1);
        let operator = match first {
            "+" => "gt",
            "-" => "lt",
            _  => "eq",
        };
        let d: i32 = match last.parse::<i32>() {
            Ok(n) => n.abs(),
            Err(_) => -1,
        };
        let condition = Condition(operator.to_string(), d);
        condition
    } else {
        let operator = "eq".to_string();
        let d: i32 = match p.parse::<i32>() {
            Ok(n) => n.abs(),
            Err(_) => -1,
        };
        let condition = Condition(operator.to_string(), d);
        condition
    }
}

fn main() {
    // Command line parameters.
    let matches = App::new("find")
        .version("0.1")
        .usage("find [-f path] path ...")
        .about("rust-version of the find-command")
        .author("Claus Guttesen")
        .arg(Arg::with_name("path")
            .multiple(true)
        )
        .arg(Arg::with_name("file hierarchy")
            .help("search-path to traverse")
            .required(false)
            .takes_value(true)
            .short("f")
            .multiple(true)
            .conflicts_with("path")
        )
        .arg(Arg::with_name("sort")
            .help("alphabetical sort")
            .required(false)
            .takes_value(false)
            .short("s")
            .long("sort")
            .multiple(false)
        )
        .arg(Arg::with_name("Bmin")
            .help("Difference between file's inode creation and time find started")
            .required(false)
            .takes_value(true)
            .multiple(false)
            .max_values(1)
            .allow_hyphen_values(true)
            .value_name("[+|-]n")
            .long("Bmin")
        )
        .get_matches();

    let sort: bool = matches.is_present("sort");
    let condition: Condition = Condition("a".to_string(), 1);
    let arguments = Arguments::new(Some(sort), Some(condition));

    if matches.is_present("Bmin") {
        let p = matches.value_of("Bmin").unwrap();
        let condition: Condition = split_parameter(p);
        let arguments = Arguments::new(Some(sort), Some(condition));
    }

    if matches.is_present("file hierarchy") {
        let paths: Vec<_> = matches.values_of("file hierarchy").unwrap().collect();
        for path in paths {
            let _r = f(path.to_string(), &arguments);
        }
    } else if matches.is_present("path") {
        let paths: Vec<_> = matches.values_of("path").unwrap().collect();
        for path in paths {
            let _r = f(path.to_string(), &arguments);
        }
    } else {
        println!("{}", matches.usage());
    }
}
