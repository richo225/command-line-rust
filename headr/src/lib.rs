use std::error::Error;

#[derive(Debug)]
pub struct Config;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {})
}
