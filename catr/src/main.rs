use clap::{App, Arg};

fn main() {
    // cargo run -- file1 file2 file3
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
        .get_matches();

    println!("{:#?}", matches)
}
