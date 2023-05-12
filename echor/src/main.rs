use clap::App;

fn main() {
    App::new("echor")
        .version("0.1.0")
        .author("Richard Bates <rich.bates@protonmail.com>")
        .about("Rust implementation of echo")
        .get_matches();
}
