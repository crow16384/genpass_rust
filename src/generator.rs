use crate::config::{Config, PassElements};
use rand::{rngs::ThreadRng, thread_rng, Rng};

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
    fn gen_word(&mut self, len: u8, upcase: bool) -> String {
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

        if upcase {
            word[0].make_ascii_uppercase()
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
                Ok(UWord(d)) => password.push(self.gen_word(d, true)),
                Ok(Word(d)) => password.push(self.gen_word(d, false)),
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
