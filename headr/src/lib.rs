use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg, ArgMatches};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Richard Bates <rich.bates@protonmail.com")
        .about("Reads specific number of file bytes and writes to output")
        .arg(
            Arg::with_name("files")
                .value_name("FILE PATH")
                .help("Provide file to read")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_of_lines")
                .long("lines")
                .short("n")
                .help("Number of lines to print")
                .default_value("10")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_of_bytes")
                .long("bytes")
                .short("c")
                .help("Number of bytes to print"),
        )
        .get_matches();

    return_config(&matches)
}

pub fn run(config: &Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                let mut line = String::new();
                for _ in 0..config.lines {
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                    line.clear();
                }
            }
        }
    }

    Ok(())
}

fn return_config(matches: &ArgMatches) -> MyResult<Config> {
    let files: Vec<String> = matches.values_of_lossy("files").unwrap();

    let lines: usize = matches
        .value_of("number_of_lines")
        // unpacks &str from Option<&str> and passes it to parse_string_to_int()
        .map(parse_string_to_int)
        // turns Option<Result<usize, Box<_>>> into Result<Option<usize>, Box<_>>
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();

    let bytes: Option<usize> = matches
        .value_of("number_of_bytes")
        .map(parse_string_to_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn parse_string_to_int(string: &str) -> MyResult<usize> {
    match string.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(string.into()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
