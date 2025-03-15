use chrono::{DateTime, Local};
use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

struct Data {
    files: Vec<PathBuf>,
    creation_bool: bool,
    modif_bool: bool,
    o_format: String,
    units: String,
}

fn datetime_print(res: Result<SystemTime, Error>, opts: &Data) {
    match res {
        Ok(result) => {
            if opts.units == "e" {
                let epochtime = result.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                print!("{} ", epochtime)
            } else if opts.units == "h" {
                let datetime: DateTime<Local> = result.into();
                print!("{} ", datetime.format(&opts.o_format))
            } else if opts.units == "a" {
                let datetime: DateTime<Local> = result.into();
                let epochtime = result.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                print!("{} {} ", epochtime, datetime.format(&opts.o_format));
            }
        }
        Err(_) => {
            print!("");
        }
    }
}

fn file_modification_time(data: &Data) {
    for filename in &data.files {
        let met = fs::metadata(filename);
        match met {
            Ok(metadata) => {
                print!("{} ", filename.to_str().unwrap());
                if data.modif_bool {
                    let modif = metadata.modified();
                    datetime_print(modif, data);
                }
                if data.creation_bool {
                    let modif = metadata.created();
                    datetime_print(modif, data);
                }
            }
            Err(_) => {
                println!("{} === file-metadata-not-found", filename.to_str().unwrap());
            }
        }
        println!()
    }
}

fn argparse() -> Data {
    let matches = Command::new("File stat")
        .author("Sabarish, github.com/sabarish-vm")
        .about("An alternative to stat command written in rust an os-independent solution")
        .arg(Arg::new("paths").action(ArgAction::Append).required(true))
        .arg(
            Arg::new("modif_need")
                .short('m')
                .help("Flag to enable outputting of Modification time")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("created_need")
                .short('c')
                .help("Flag to enable outputting of Creation time")
                .action(ArgAction::SetTrue),
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
    let modif_bool = matches.get_flag("modif_need");
    let creation_bool = matches.get_flag("created_need");
    let o_format: String = matches.get_one::<String>("format").unwrap().into();
    let units = matches.get_one::<String>("units").unwrap().to_owned();
    Data {
        modif_bool,
        creation_bool,
        files,
        o_format,
        units,
    }
}

fn main() {
    let data = argparse();
    file_modification_time(&data);
}
