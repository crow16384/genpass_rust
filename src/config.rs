use clap::{
    crate_authors, crate_description, crate_version, value_parser, Arg, ArgAction, ArgMatches,
    Command,
};
use std::convert::TryFrom;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

const MAX_ELEMENT_LENGTH: usize = 255;
const LAST_FORMAT_FILE: &str = ".genpass_memory";

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum PassElements {
    Word(usize),    // Readable words
    UWord(usize),   // Readable words started with upper case letter
    PWord(usize),   // Pronounceable words
    UPWord(usize),  // Pronounceable words started with upper case letter
    Digits(usize),  // Digits
    Special(usize), // Special symbols
}

impl PartialEq for PassElements {
    fn eq(&self, other: &Self) -> bool {
        use PassElements::*;

        match (self, other) {
            (Word(a), Word(b))
            | (UWord(a), UWord(b))
            | (PWord(a), PWord(b))
            | (UPWord(a), UPWord(b))
            | (Digits(a), Digits(b))
            | (Special(a), Special(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for PassElements {}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("no format provided and no remembered format found (use format or --last)")]
    MissingFormat,
    #[error("invalid element type (first character): {0}")]
    InvalidElementType(char),
    #[error("invalid element length provided: {0}")]
    ParseElementLengthError(#[from] std::num::ParseIntError),
    #[error("element length can't be '0'")]
    ZeroElementLength,
    #[error("max element length exceeded ({0})", MAX_ELEMENT_LENGTH)]
    MaxElementLengthError,
}

impl TryFrom<&String> for PassElements {
    type Error = ConfigError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        let element_type = chars.next().ok_or(ConfigError::MissingFormat)?;
        let len = if value.len() == 1 {
            1
        } else {
            value[1..].parse::<usize>()?
        };

        if len == 0 {
            return Err(ConfigError::ZeroElementLength);
        }

        if len > MAX_ELEMENT_LENGTH {
            return Err(ConfigError::MaxElementLengthError);
        }

        match element_type {
            'W' => Ok(Self::UWord(len)),
            'w' => Ok(Self::Word(len)),
            'P' => Ok(Self::UPWord(len)),
            'p' => Ok(Self::PWord(len)),
            'd' => Ok(Self::Digits(len)),
            's' => Ok(Self::Special(len)),
            c => Err(ConfigError::InvalidElementType(c)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub format: Vec<Result<PassElements, ConfigError>>,
    pub quantity: u32,
    pub raw_format: String,
}

fn memory_path() -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    if home.is_empty() {
        return None;
    }
    Some(PathBuf::from(home).join(LAST_FORMAT_FILE))
}

fn read_last_format() -> Option<String> {
    let path = memory_path()?;
    let format = fs::read_to_string(path).ok()?;
    let trimmed = format.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn save_last_format(format: &str) {
    if let Some(path) = memory_path() {
        let _ = fs::write(path, format);
    }
}

fn parse_elements(value: &str) -> Vec<Result<PassElements, ConfigError>> {
    if value.is_empty() {
        return vec![Err(ConfigError::MissingFormat)];
    }

    let chars: Vec<char> = value.chars().collect();
    let mut idx = 0;
    let mut result = Vec::new();

    while idx < chars.len() {
        let start = idx;
        idx += 1;

        while idx < chars.len() && chars[idx].is_ascii_digit() {
            idx += 1;
        }

        let part: String = chars[start..idx].iter().collect();
        result.push(PassElements::try_from(&part));
    }

    result
}

impl Config {
    fn command() -> Command {
        Command::new("genpass")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .value_parser(value_parser!(u32))
                    .action(ArgAction::Set)
                    .value_name("N")
                    .default_value("3")
                    .required(false)
                    .help("Number of passwords to be generated"),
            )
            .arg(
                Arg::new("last")
                    .short('l')
                    .long("last")
                    .action(ArgAction::SetTrue)
                    .help("Use the last remembered format"),
            )
            .arg(
                Arg::new("format")
                    .action(ArgAction::Set)
                    .value_name("FORMAT")
                    .required(false)
                    .help(
                        "Password format in compact form (example: W4s2w3d5, p8d2). \
                         Segment length defaults to 1 when omitted.",
                    ),
            )
    }

    fn from_matches(matches: ArgMatches) -> Self {
        let use_last = matches.get_flag("last");
        let mut raw_format = matches
            .get_one::<String>("format")
            .map_or_else(String::new, ToString::to_string);

        if raw_format.is_empty() && use_last {
            raw_format = read_last_format().unwrap_or_default();
        }

        let format = parse_elements(&raw_format);

        let quantity: u32 = match matches.get_one("number") {
            Some(d) => *d,
            None => 3,
        };

        Config {
            format,
            quantity,
            raw_format,
        }
    }

    fn from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Self::command().get_matches_from(args);
        Self::from_matches(matches)
    }

    /// Parse a command line and return Result with Config
    pub fn new() -> Self {
        Self::from_args(env::args_os())
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
            eprintln!("  where x could be 'w', 'W', 'p', 'P', 'd', 's'");
            eprintln!("        n - optional element length (defaults to 1)");
            eprintln!("  MAX element's length = {}", MAX_ELEMENT_LENGTH);
            eprintln!("\n\nExample: genpass W4s2w3d5");
            eprintln!("Or use last: genpass --last");
            eprintln!("========");
            eprintln!("Will produce something like: Cyvi!:wof90943");
            std::process::exit(1);
        }

        save_last_format(&self.raw_format);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_home_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        env::temp_dir().join(format!(
            "genpass-test-home-{}-{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn parse_compact_format_success() {
        let parsed = parse_elements("W4s2w3d5");

        assert_eq!(parsed.len(), 4);
        assert!(matches!(parsed[0], Ok(PassElements::UWord(4))));
        assert!(matches!(parsed[1], Ok(PassElements::Special(2))));
        assert!(matches!(parsed[2], Ok(PassElements::Word(3))));
        assert!(matches!(parsed[3], Ok(PassElements::Digits(5))));
    }

    #[test]
    fn parse_default_lengths_success() {
        let parsed = parse_elements("Wpds");

        assert_eq!(parsed.len(), 4);
        assert!(matches!(parsed[0], Ok(PassElements::UWord(1))));
        assert!(matches!(parsed[1], Ok(PassElements::PWord(1))));
        assert!(matches!(parsed[2], Ok(PassElements::Digits(1))));
        assert!(matches!(parsed[3], Ok(PassElements::Special(1))));
    }

    #[test]
    fn parse_invalid_type_is_reported() {
        let parsed = parse_elements("x3");

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0],
            Err(ConfigError::InvalidElementType('x'))
        ));
    }

    #[test]
    fn parse_zero_and_max_length_validation() {
        let zero = parse_elements("w0");
        let too_big = parse_elements("d256");

        assert!(matches!(zero[0], Err(ConfigError::ZeroElementLength)));
        assert!(matches!(
            too_big[0],
            Err(ConfigError::MaxElementLengthError)
        ));
    }

    #[test]
    fn cli_uses_last_format_when_requested() {
        let home = unique_home_dir();
        fs::create_dir_all(&home).expect("must create test home directory");
        fs::write(home.join(LAST_FORMAT_FILE), "p8d2\n").expect("must write last format file");

        env::set_var("HOME", &home);
        let cfg = Config::from_args(["genpass", "--last", "-n", "2"]);

        assert_eq!(cfg.quantity, 2);
        assert_eq!(cfg.raw_format, "p8d2");
        assert_eq!(cfg.format.len(), 2);
        assert!(matches!(cfg.format[0], Ok(PassElements::PWord(8))));
        assert!(matches!(cfg.format[1], Ok(PassElements::Digits(2))));

        let _ = fs::remove_file(home.join(LAST_FORMAT_FILE));
        let _ = fs::remove_dir(home);
    }
}
