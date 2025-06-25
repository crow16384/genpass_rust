use crate::config::{Config, PassElements};
use rand::{prelude::IndexedRandom, rng, rngs::ThreadRng};

/// Generator structure. It contains only random number generator thread
pub struct Generator {
    rng: ThreadRng,
}

/// Static arrays for fast getting
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

/// Upcase the first letter in the string
fn uppercase_first(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

impl Generator {
    /// Mainly it's just a generator thread from rand package
    pub fn new() -> Self {
        Generator { rng: rng() }
    }

    /// Implement a `word` generation
    fn gen_word(&mut self, len: usize, upcase: bool) -> String {
        let mut word = String::with_capacity(len);

        for i in 0..len {
            if i % 2 == 0 {
                if let Some(c) = CONSONANTS.choose(&mut self.rng) {
                    word.push(*c);
                }
            } else if let Some(c) = VOWELS.choose(&mut self.rng) {
                word.push(*c);
            }
        }
        if upcase {
            uppercase_first(&word)
        } else {
            word
        }
    }

    /// Implement a `digits` generation
    fn gen_digits(&mut self, len: usize) -> String {
        let mut digits = String::with_capacity(len);

        for _ in 0..len {
            if let Some(d) = DIGITS.choose(&mut self.rng) {
                digits.push(*d);
            }
        }
        digits
    }

    /// Implement a `special chars` generation
    fn gen_special(&mut self, len: usize) -> String {
        let mut spec = String::with_capacity(len);

        for _ in 0..len {
            if let Some(d) = SPECIAL.choose(&mut self.rng) {
                spec.push(*d);
            }
        }
        spec
    }

    pub fn run(&mut self, elements: Config) -> Vec<String> {
        use PassElements::*;

        let mut passwords: Vec<String> = vec![]; // all generated passwords
        let mut password: Vec<String> = vec![]; // single password during generation

        let n = elements.quantity;

        for _ in 0..n {
            for e in &elements.format {
                match e {
                    Ok(UWord(d)) => password.push(self.gen_word(*d, true)),
                    Ok(Word(d)) => password.push(self.gen_word(*d, false)),
                    Ok(Digits(d)) => password.push(self.gen_digits(*d)),
                    Ok(Special(d)) => password.push(self.gen_special(*d)),
                    _ => (),
                }
            }
            let p = password.join("");
            passwords.push(p);
            password.clear();
        }
        passwords
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
