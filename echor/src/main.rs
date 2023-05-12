use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Richard Bates <rich.bates@protonmail.com>")
        .about("Rust implementation of echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input some text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Don't print new lines")
                .takes_value(false),
        )
        .get_matches();

    let omit_new_line: bool = matches.is_present("omit_newline");
    let ending: &str = if omit_new_line { "" } else { "\n" };

    let text: Vec<String> = matches.values_of_lossy("text").unwrap();
    println!("{}{}", text.join(" "), ending);
}
