use clap::{ArgAction, Command, Arg};
use std::error::Error;

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum Password {
    Word(u8),       // Readable words
    Digits(u8),     // Digits
    Special(u8),    // Special symbols
    Any(u8),        // Any character, digit or special symbol
    FormatError     // Incorrect format
}

/// Return optionat u8 digit from string
fn get_digits (value: &[char]) -> Option<u8> {
    let digs: String = value.iter().collect();
    digs.parse::<u8>().ok()
}

impl From<String> for Password {
    fn from(value: String) -> Self {        
        let val: Vec<char> = value.chars().collect();
        match &val[..] {
            ['w', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => Password::Word(d),
                    None => Password::FormatError,
                }
            },
            ['d', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => Password::Digits(d),
                    None => Password::FormatError,
                }
            },
            ['s', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => Password::Special(d),
                    None => Password::FormatError,
                }
            },
            ['a', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => Password::Any(d),
                    None => Password::FormatError,
                }
            },
            _ => Self::FormatError,
        }
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    format: Vec<Password>, 
}


/// Parse a command line and return Result with Config
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

    let fmt: Vec<String> = matches
                .get_many::<String>("format")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect();
    println!("{:?}",fmt);

    Ok(Config { format: vec![Password::Word(4), Password::Special(1), Password::Digits(4)] })
}

