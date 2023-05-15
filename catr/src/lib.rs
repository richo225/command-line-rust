use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Config {
    number_all_lines: bool,
    number_nonblank_lines: bool,
    files: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Richard Bates <rich.bates@protonmail.com")
        .about("Reads files sequentially and writes to output")
        .arg(
            Arg::with_name("files")
                .value_name("FILE PATH")
                .help("Provide file(s) to read")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_all_lines")
                .long("number")
                .short("n")
                .help("Number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .long("number-nonblank")
                .short("b")
                .help("Number only non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_all_lines: matches.is_present("number_all_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: &Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_number = 0;

                for (line_number, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if config.number_all_lines {
                        println!("{:>6}\t{}", line_number + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_number += 1;
                            println!("{:>6}\t{}", last_number, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
