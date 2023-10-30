use clap::{crate_authors, crate_description, crate_version, Arg, ArgAction, Command};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::convert::TryFrom;
use thiserror::Error;

const MAX_WORD_LENGTH: u8 = 10;
//const PRG: &str = "genpass";

/// Parts of the password to be constructed
#[derive(Debug)]
enum PassElements {
    Word(u8),    // Readable words
    Digits(u8),  // Digits
    Special(u8), // Special symbols
    Any(u8),     // Any character, digit or special symbol
}

#[derive(Debug, Error)]
enum ConfigError {
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
    format: Vec<Result<PassElements, ConfigError>>,
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

pub struct Generator {
    rng: ThreadRng,
}

static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
static CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'z',
];
static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static SPECIAL: [char; 25] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '~', '>', '<', '(', ')', '\\', '/', ',', '=', ';', ':',
    '+', '-', '.', '[', ']', '_',
];

impl Generator {
    /// Mainly it's just a generator thread from rand package
    pub fn new() -> Self {
        Generator { rng: thread_rng() }
    }

    /// Implement a `word` generation
    fn gen_word(&mut self, len: u8) -> String {
        let mut word: Vec<char> = vec![];

        for i in 0..len {
            if i % 2 == 0 {
                let idx = self.rng.gen_range(0..CONSONANTS.len());
                word.push(CONSONANTS[idx]);
            } else {
                let idx = self.rng.gen_range(0..VOWELS.len());
                word.push(VOWELS[idx]);
            }
        }

        word.into_iter().collect()
    }

    /// Implement a `digits` generation
    fn gen_digits(&mut self, len: u8) -> String {
        let mut digits: Vec<char> = vec![];

        for _ in 0..len {
            let idx = self.rng.gen_range(0..DIGITS.len());
            digits.push(DIGITS[idx]);
        }

        digits.into_iter().collect()
    }

    /// Implement a `special chars` generation
    fn gen_special(&mut self, len: u8) -> String {
        let mut spec: Vec<char> = vec![];

        for _ in 0..len {
            let idx = self.rng.gen_range(0..SPECIAL.len());
            spec.push(SPECIAL[idx]);
        }

        spec.into_iter().collect()
    }

    pub fn run(&mut self, elements: Config) -> String {
        use PassElements::*;

        let mut password: Vec<String> = vec![];

        for e in elements.format {
            match e {
                Ok(Word(d)) => password.push(self.gen_word(d)),
                Ok(Digits(d)) => password.push(self.gen_digits(d)),
                Ok(Special(d)) => password.push(self.gen_special(d)),
                _ => (),
            }
        }
        password.join("")
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

/*#[cfg(test)]
mod test {
    use super::{Config, PassElements};
    use std::convert::TryFrom;

    fn convert_valid_element() {
        let expected = Config { format: vec![Ok(PassElements::Word(8))] };
        let actual = Config::try_from(&String::from("w8"));
        assert!(actual.is_ok(), "valid element should be converted to Config");
        //assert_eq!(actual.unwrap(), expected, "wrong element value");
    }
}*/
