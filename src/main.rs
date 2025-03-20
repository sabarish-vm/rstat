use chrono::{DateTime, Local};
use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum CliError {
    ErrorRstat,
}

impl From<std::io::Error> for CliError {
    fn from(_: std::io::Error) -> CliError {
        CliError::ErrorRstat
    }
}

struct Opts {
    files: Vec<PathBuf>,
    creation_bool: bool,
    modif_bool: bool,
    access_bool: bool,
    o_format: String,
    units: String,
}

struct MutOpts {
    errored: bool,
}

fn datetime_print(res: Result<SystemTime, Error>, opts: &Opts, mut_opts: &mut MutOpts) {
    match res {
        Ok(result) => {
            if opts.units == "e" {
                let epochtime = result.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                print!("{} ; ", epochtime)
            } else if opts.units == "h" {
                let datetime: DateTime<Local> = result.into();
                print!("{} ; ", datetime.format(&opts.o_format))
            } else if opts.units == "a" {
                let datetime: DateTime<Local> = result.into();
                let epochtime = result.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                print!("{} {} ; ", epochtime, datetime.format(&opts.o_format));
            }
        }
        Err(_) => {
            mut_opts.errored = true;
            print!("");
        }
    }
}

fn file_modification_time(opts: &Opts, mut_opts: &mut MutOpts) {
    for filename in &opts.files {
        let met = fs::metadata(filename);
        match met {
            Ok(metadata) => {
                print!("{} ; ", filename.to_str().unwrap());
                if opts.modif_bool {
                    let modif = metadata.modified();
                    datetime_print(modif, opts, mut_opts);
                }
                if opts.creation_bool {
                    let modif = metadata.created();
                    datetime_print(modif, opts, mut_opts);
                }
                if opts.access_bool {
                    let modif = metadata.accessed();
                    datetime_print(modif, opts, mut_opts);
                }
            }
            Err(err) => {
                mut_opts.errored = true;
                println!("{} {}", filename.to_str().unwrap(), err);
            }
        }
        println!()
    }
}

fn argparse() -> (Opts, MutOpts) {
    let matches = Command::new("File stat")
        .author("Sabarish, github.com/sabarish-vm")
        .about("An alternative to stat command written in rust an os-independent solution")
        .arg(Arg::new("paths").action(ArgAction::Append).required(true))
        .arg(
            Arg::new("created_need")
                .short('c')
                .help("Flag to enable outputting of Creation time")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("modif_need")
                .short('m')
                .help("Flag to enable outputting of Modification time")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("access_need")
                .short('a')
                .help("Flag to enable outputting of Accessed time")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("units")
                .short('u')
                .value_parser(["e", "h", "a"])
                .default_value("h")
                .help(
                    "Choose units.\
                \n* e to use Unix-Epoch-time as units, i.e.,\
                 time elapsed in seconds since 1970-01-01 00:00:00 (GMT)\
                \n* h to print time in terms of human readable units. \
                Can be further customized using -o flag\
                \n* a to print both of them seperated by a space",
                ),
        )
        .arg(
            Arg::new("format")
                .short('o')
                .help("Output formmating style")
                .default_value("%Y-%m-%d %H:%M:%S"),
        )
        .get_matches();
    let files = matches
        .get_many::<String>("paths")
        .unwrap()
        .map(|s| PathBuf::from_str(s).unwrap())
        .collect::<Vec<PathBuf>>();
    let mut modif_bool: bool = matches.get_flag("modif_need");
    let mut creation_bool: bool = matches.get_flag("created_need");
    let mut access_bool: bool = matches.get_flag("access_need");
    if !modif_bool && !creation_bool && !access_bool {
        modif_bool = true;
        creation_bool = true;
        access_bool = true;
    }
    let o_format: String = matches.get_one::<String>("format").unwrap().into();
    let units = matches.get_one::<String>("units").unwrap().to_owned();
    (
        Opts {
            modif_bool,
            creation_bool,
            access_bool,
            files,
            o_format,
            units,
        },
        MutOpts { errored: false },
    )
}

fn main() -> Result<(), CliError> {
    let (opts, mut mutopts) = argparse();
    file_modification_time(&opts, &mut mutopts);
    if mutopts.errored {
        Err(CliError::ErrorRstat)
    } else {
        Ok(())
    }
}
