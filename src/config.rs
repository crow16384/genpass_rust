use clap::{crate_authors, crate_description, crate_version, Arg, ArgAction, Command};
use std::convert::TryFrom;
use thiserror::Error;

const MAX_WORD_LENGTH: u8 = 10;
//const PRG: &str = "genpass";

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum PassElements {
    Word(u8),    // Readable words
    Digits(u8),  // Digits
    Special(u8), // Special symbols
    Any(u8),     // Any character, digit or special symbol
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid element type (first character): {0}")]
    InvalidElementType(char),
    #[error("invalid element whole length (must be 2 or 3)")]
    InvalidTotalLength,
    #[error("invalid element length provided: {0}")]
    ParseElementLengthError(#[from] std::num::ParseIntError),
    #[error("element length can't be '0'")]
    ZeroElementLength,
    #[error("max element length exceede ({0})", MAX_WORD_LENGTH)]
    MaxElementLengthError,
}

impl TryFrom<&String> for PassElements {
    type Error = ConfigError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.len() < 2 || value.len() > 3 {
            return Err(ConfigError::InvalidTotalLength);
        }

        let val: Vec<char> = value.chars().collect();
        let valid: Vec<char> = vec!['w', 'd', 's', 'a'];

        if !valid.contains(&val[0]) {
            return Err(ConfigError::InvalidElementType(val[0]));
        }

        let d = value[1..].parse::<u8>()?;

        if d == 0 {
            return Err(ConfigError::ZeroElementLength);
        }

        if d > MAX_WORD_LENGTH {
            return Err(ConfigError::MaxElementLengthError);
        }

        match &val[0] {
            'w' => Ok(Self::Word(d)),
            'd' => Ok(Self::Digits(d)),
            's' => Ok(Self::Special(d)),
            'a' => Ok(Self::Any(d)),
            c => Err(ConfigError::InvalidElementType(*c)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub format: Vec<Result<PassElements, ConfigError>>,
}

impl Config {
    /// Parse a command line and return Result with Config
    pub fn new() -> Self {
        let matches = Command::new("genpass")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::new("format")
                    .action(ArgAction::Append)
                    .value_name("FORMAT")
                    .help("Specify the password format")
                    .default_value("w4 s1 d4"),
            )
            .get_matches();

        let fmt: Vec<Result<PassElements, ConfigError>> = matches
            .get_many::<String>("format")
            .unwrap_or_default()
            .map(PassElements::try_from)
            .collect();

        Config { format: fmt }
    }

    /// Check the Config. FormatError items must be pointed out to user
    /// if any. If password elements are fine then return Config for the further
    /// processing.
    pub fn check(self) -> Self {
        let mut bad_fmt_indx = vec![];
        let mut error_flag = false;

        for (pos, el) in self.format.iter().enumerate() {
            match el {
                Err(er) => {
                    eprintln!("Error: {}\n\n", er);
                    bad_fmt_indx.push(pos + 1);
                    error_flag = true;
                }
                Ok(_) => (),
            }
        }

        if error_flag {
            eprint!("Error in password element(s) ##: ");
            for i in bad_fmt_indx {
                eprint!("{} ", i);
            }
            eprintln!("\n\nFormat: [x][d]");
            eprintln!("  where x could be 'w' (word),'d' (digits),");
            eprintln!("                   'a' (any char),'s' (special)");
            eprintln!("        d - length of the element");
            eprintln!("  MAX element length = {}", MAX_WORD_LENGTH);
            eprintln!("\n\nExample: genpass w4 s2 d3");
            eprintln!("Will produce like: Dihu#?123");
            std::process::exit(1);
        }
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
