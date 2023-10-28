use clap::{ArgAction, Command, Arg};
use std::error::Error;

/// Parts of the password to be constructed
#[derive(Debug)]
enum Password {
    Word(u8),
    Digits(u8),
    Special(u8),
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    format: Vec<Password>, 
}

pub fn get_config() -> MyResult<Config> {
    let matches = Command::new("genpass")
            .version("0.1.0")
            .author("Crow16384 <crow16384@yandex.ru>")
            .about("Password generator")
            .arg(
                Arg::new("format")
                    .action(ArgAction::Append)
                    .value_name("FORMAT")
                    .help("Specify the password format")
                    .default_value("w4 s1 d4"),
            )
            .get_matches();
    
    Ok(Config { format: vec![Password::Word(4), Password::Special(1), Password::Digits(4)] })
}

