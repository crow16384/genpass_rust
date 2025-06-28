use clap::{
    crate_authors, crate_description, crate_version, value_parser, Arg, ArgAction, Command,
};
use std::convert::TryFrom;
use thiserror::Error;

const MAX_WORD_LENGTH: usize = 10;

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum PassElements {
    Word(usize),    // Readable words
    UWord(usize),   // Readable words started with upper case letter
    Digits(usize),  // Digits
    Special(usize), // Special symbols
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
    #[error("max element length exceeded ({0})", MAX_WORD_LENGTH)]
    MaxElementLengthError,
}

impl TryFrom<&String> for PassElements {
    type Error = ConfigError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.len() < 2 || value.len() > 3 {
            return Err(ConfigError::InvalidTotalLength);
        }

        let val: Vec<char> = value.chars().collect();
        let valid: Vec<char> = vec!['W', 'w', 'd', 's'];

        if !valid.contains(&val[0]) {
            return Err(ConfigError::InvalidElementType(val[0]));
        }

        let d = value[1..].parse::<usize>()?;

        if d == 0 {
            return Err(ConfigError::ZeroElementLength);
        }

        if d > MAX_WORD_LENGTH {
            return Err(ConfigError::MaxElementLengthError);
        }

        match &val[0] {
            'W' => Ok(Self::UWord(d)),
            'w' => Ok(Self::Word(d)),
            'd' => Ok(Self::Digits(d)),
            's' => Ok(Self::Special(d)),
            c => Err(ConfigError::InvalidElementType(*c)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub format: Vec<Result<PassElements, ConfigError>>,
    pub quantity: u32,
}

impl Config {
    /// Parse a command line and return Result with Config
    pub fn new() -> Self {
        let matches = Command::new("genpass")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::new("count")
                    .short('n')
                    .long("quantity")
                    .value_parser(value_parser!(u32))
                    .action(ArgAction::Set)
                    .value_name("COUNT")
                    .default_value("1")
                    .required(false)
                    .help("Number (quantity) of passwords to be generated"),
            )
            .arg(
                Arg::new("format")
                    .action(ArgAction::Append)
                    .value_name("FORMAT")
                    .required(true)
                    .value_delimiter('_')
                    .help(
                        r#"Specify the password format [x][n]_...
   where x could be:
        'w' (word),
        'W' (word's first letter is upcased),
        'd' (digits), 's' (special chars).

        n - length of the element.

   Example: genpass W4_s2_d3 ==> something like: Dihu#?123"#,
                    ),
            )
            .get_matches();

        let format: Vec<Result<PassElements, ConfigError>> = matches
            .get_many("format")
            .unwrap_or_default()
            .map(PassElements::try_from)
            .collect();

        let quantity: u32 = match matches.get_one("count") {
            Some(d) => *d,
            None => 1,
        };

        Config { format, quantity }
    }

    /// Check the Config. FormatError items must be pointed out to user
    /// if any. If password elements are fine then return Config for the further
    /// processing.
    pub fn check(self) -> Self {
        let mut bad_fmt_index = vec![];
        let mut error_flag = false;

        for (pos, el) in self.format.iter().enumerate() {
            if let Err(er) = el {
                eprintln!("Error: {}\n\n", er);
                bad_fmt_index.push(pos + 1);
                error_flag = true;
            }
        }

        if error_flag {
            eprint!("Incorrect password element(s) ##: ");
            for i in bad_fmt_index {
                eprint!("{} ", i);
            }
            eprintln!("\n\nFormat: [x][n]");
            eprintln!("  where x could be 'w' (word),'W' (word's first letter is upcased),");
            eprintln!("                   'd' (digits), 's' (special chars)");
            eprintln!("        n - length of the element");
            eprintln!("  MAX element's length = {}", MAX_WORD_LENGTH);
            eprintln!("\n\nExample: genpass W4_s2_d3");
            eprintln!("========");
            eprintln!("Will produce something like: Dihu#?123");
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
