use chrono::{DateTime, Local};
use std::fs;
use std::time::UNIX_EPOCH;

fn file_modification_time(filename: &str) {
    let met = fs::metadata(filename);
    match met {
        Ok(metadata) => {
            let modif = metadata.modified();
            match modif {
                Ok(result) => {
                    let epochtime = result.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                    let datetime: DateTime<Local> = result.into();
                    println!(
                        "{} {} {}",
                        filename,
                        epochtime,
                        datetime.format("%Y-%m-%d %H:%M:%S")
                    );
                }
                Err(err) => {
                    eprintln!("Error retrieving modified time for file {filename}: {err}",);
                }
            }
        }
        Err(err) => {
            eprintln!("Error retrieving metadata for file {}: {}", filename, err);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <file1> <file2> ... <fileN>", args[0]);
    }

    for filename in &args[1..] {
        file_modification_time(filename);
    }
}
