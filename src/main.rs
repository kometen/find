extern crate clap;

use clap::{Arg, App};
use std::{fs, io};
use std::process::exit;
use std::time::SystemTime;

fn f(path: String, arguments: &Arguments) -> io::Result<()> {
    let sys_time = SystemTime::now();

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
            let metadata = fs::metadata(&entry)?;
            if let Ok(time) = metadata.created() {
                let file_seconds = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let file_minutes = file_seconds / 60;
                let system_minute = sys_time.duration_since(SystemTime::UNIX_EPOCH).unwrap() / 60;
                if system_minute.as_secs() == (file_minutes.as_secs() + arguments.bmin.1 as u64) {
                    println!("seconds: {}, minutes: {}, sys-minutes: {}", file_seconds.as_secs(), file_minutes.as_secs(), system_minute.as_secs());
                    println!("{}", &entry.display());
                }
            }
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
            Err(_) => {
                eprintln!("Unable to parse parameter, exiting");
                exit(-1);
            },
        };
        let condition = Condition(operator.to_string(), d);
        condition
    } else {
        let (o, d) = match p.parse::<i32>() {
            Ok(n) => {
                ("eq".to_string(), n.abs())
            },
            Err(_) => {
                eprintln!("Unable to parse parameter, exiting");
                exit(-1);
            },
        };
        let condition = Condition(o, d);
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

    let sort = matches.is_present("sort");

    let bmin_condition= match matches.value_of("Bmin") {
        Some(bmin_condition) => {
            let c = split_parameter(matches.value_of("Bmin").unwrap());
            c
        },
        None => {
            let c = split_parameter("-1");
            c
        },
    };
    println!("bmin.0: {}, bmin.1: {}", bmin_condition.0, bmin_condition.1);

    let arguments = Arguments::new(Some(sort), Some(bmin_condition));

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
