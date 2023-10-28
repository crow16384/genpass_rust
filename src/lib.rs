use clap::{ArgAction, Command, Arg};

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum PassElements {
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

impl From<&String> for PassElements {
    fn from(value: &String) -> Self {        
        let val: Vec<char> = value.chars().collect();
        match &val[..] {
            ['w', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => PassElements::Word(d),
                    None => PassElements::FormatError,
                }
            },
            ['d', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => PassElements::Digits(d),
                    None => PassElements::FormatError,
                }
            },
            ['s', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => PassElements::Special(d),
                    None => PassElements::FormatError,
                }
            },
            ['a', digs @ ..] => {
                match get_digits(digs) {
                    Some(d) => PassElements::Any(d),
                    None => PassElements::FormatError,
                }
            },
            _ => Self::FormatError,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub format: Vec<PassElements>, 
}

/// Parse a command line and return Result with Config
pub fn get_config() -> Config {
    let matches = Command::new("genpass")
            .version("0.1.0")
            .author("Crow16384 <crow16384@yandex.ru>")
            .about("PassElements generator")
            .arg(
                Arg::new("format")
                    .action(ArgAction::Append)
                    .value_name("FORMAT")
                    .help("Specify the password format")
                    .default_value("w4 s1 d4"),
            )
            .get_matches();

    let fmt: Vec<PassElements> = matches
                .get_many::<String>("format")
                .unwrap_or_default()
                .map(|v| PassElements::from(v))
                .collect();

    Config { format: fmt }
}
