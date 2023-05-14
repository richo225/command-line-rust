use clap::{App, Arg};
use std::fs;

#[derive(Debug)]
struct Config {
    number_all_lines: bool,
    number_nonblank_lines: bool,
    files: Vec<String>,
}

fn main() {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Richard Bates <rich.bates@protonmail.com")
        .about("Reads files sequentially and writes to output")
        .arg(
            Arg::with_name("file_name")
                .value_name("FILE_NAME")
                .help("Provide a file to read")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .long("--number")
                .short("n")
                .help("Number all output lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .long("--number-nonblank")
                .short("b")
                .help("Number only non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    let config = Config {
        files: matches.values_of_lossy("file_name").unwrap(),
        number_all_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    };

    parse_file(&config);
}

fn parse_file(config: &Config) {
    for file in &config.files {
        let content = fs::read_to_string(file);
        print!("{}", content.unwrap())
    }
}
