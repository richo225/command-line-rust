use std::error::Error;

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

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

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of_lossy("lines").unwrap(),
        bytes: matches.value_of_lossy("bytes").unwrap(),
    })
}
