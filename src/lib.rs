use clap::{crate_authors, crate_description, crate_version, Arg, ArgAction, Command};

/// Parts of the password to be constructed
#[derive(Debug)]
pub enum PassElements {
    Word(u8),    // Readable words
    Digits(u8),  // Digits
    Special(u8), // Special symbols
    Any(u8),     // Any character, digit or special symbol
    FormatError, // Incorrect format
}

/// Return optional u8 digit from string
fn get_digits(value: &[char]) -> Option<u8> {
    let digs: String = value.iter().collect();
    digs.parse::<u8>().ok()
}

impl From<&String> for PassElements {
    fn from(value: &String) -> Self {
        let val: Vec<char> = value.chars().collect();
        match &val[..] {
            ['w', digs @ ..] => match get_digits(digs) {
                Some(d) => PassElements::Word(d),
                None => PassElements::FormatError,
            },
            ['d', digs @ ..] => match get_digits(digs) {
                Some(d) => PassElements::Digits(d),
                None => PassElements::FormatError,
            },
            ['s', digs @ ..] => match get_digits(digs) {
                Some(d) => PassElements::Special(d),
                None => PassElements::FormatError,
            },
            ['a', digs @ ..] => match get_digits(digs) {
                Some(d) => PassElements::Any(d),
                None => PassElements::FormatError,
            },
            _ => Self::FormatError,
        }
    }
}

const MAX_WORD_LENGTH: u8 = 10;

#[derive(Debug)]
pub struct Config {
    format: Vec<PassElements>,
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

        let fmt: Vec<PassElements> = matches
            .get_many::<String>("format")
            .unwrap_or_default()
            .map(|v| PassElements::from(v))
            .collect();

        Config { format: fmt }
    }

    /// Check the Config. FormatError items must be pointed out to user
    /// if any. If password elements are fine then return Config for the further
    /// processing.
    pub fn check(self) -> Self {
        use PassElements::*;

        let mut bad_fmt_indx = vec![];
        let mut error_flag = false;

        for (pos, e) in self.format.iter().enumerate() {
            match e {
                FormatError => {
                bad_fmt_indx.push(pos+1);
                error_flag = true;
                },
                _ => (),
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
            eprintln!("Example: genpass w4 s2 d3");
            eprintln!("Will produce like: Dihu#?123");
            std::process::exit(1); 
        }
        self
    }
}

impl PassElements {
    /// Get length of the element for further construction
    fn len(self) -> Option<u8> {
        use PassElements::*;

        match self {
            Word(d) | Digits(d) | Special(d) | Any(d) => Some(d),
            _ => None,
        }
    }
}

#[test]
fn test_get_len() {
    use PassElements::*;

    assert_eq!(Some(11), Word(11).len());
    assert_eq!(Some(6), Special(6).len());
    assert_eq!(Some(4), Any(4).len());
    assert_eq!(Some(23), Digits(23).len());
    assert_eq!(None, FormatError.len());
}
