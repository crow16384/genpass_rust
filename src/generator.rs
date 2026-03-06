use crate::config::{Config, PassElements};
use rand::{prelude::IndexedRandom, rng, rngs::ThreadRng, Rng};

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
static CONSONANT_DIGRAPHS: [&str; 8] = ["th", "sh", "ch", "qu", "wh", "ph", "ck", "ng"];
static VOWEL_DIGRAPHS: [&str; 5] = ["oo", "ee", "ea", "ai", "oa"];
static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static SPECIAL: [char; 25] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '~', '>', '<', '(', ')', '\\', '/', ',', '=', ';', ':',
    '+', '-', '.', '[', ']', '_',
];

impl Generator {
    /// Mainly it's just a generator thread from rand package
    pub fn new() -> Self {
        Generator { rng: rng() }
    }

    /// Implement a `word` generation
    fn gen_word(&mut self, len: usize, uppercase: bool) -> String {
        let mut word: Vec<char> = Vec::with_capacity(len);

        for i in 0..len {
            let mut character: char = if i % 2 == 0 {
                *CONSONANTS.choose(&mut self.rng).unwrap_or(&'b')
            } else {
                *VOWELS.choose(&mut self.rng).unwrap_or(&'a')
            };

            if i == 0 && uppercase {
                character = character.to_ascii_uppercase();
            }
            word.push(character);
        }

        word.into_iter().collect()
    }

    fn gen_pronounceable_word(&mut self, len: usize, uppercase: bool) -> String {
        if len == 0 {
            return String::new();
        }

        let mut word = String::with_capacity(len + 2);
        let mut consonant_next = true;

        while word.len() < len {
            if consonant_next {
                if self.rng.random_bool(0.35) {
                    word.push_str(
                        CONSONANT_DIGRAPHS
                            .choose(&mut self.rng)
                            .copied()
                            .unwrap_or("th"),
                    );
                } else {
                    word.push(*CONSONANTS.choose(&mut self.rng).unwrap_or(&'b'));
                }
            } else if self.rng.random_bool(0.25) {
                word.push_str(
                    VOWEL_DIGRAPHS
                        .choose(&mut self.rng)
                        .copied()
                        .unwrap_or("oo"),
                );
            } else {
                word.push(*VOWELS.choose(&mut self.rng).unwrap_or(&'a'));
            }
            consonant_next = !consonant_next;
        }

        word.truncate(len);

        if uppercase {
            let mut chars: Vec<char> = word.chars().collect();
            if let Some(first) = chars.first_mut() {
                *first = first.to_ascii_uppercase();
            }
            return chars.into_iter().collect();
        }

        word
    }

    /// Implement a `digits` generation
    fn gen_digits(&mut self, len: usize) -> String {
        let mut digits: Vec<char> = Vec::with_capacity(len);

        for _ in 0..len {
            digits.push(*DIGITS.choose(&mut self.rng).unwrap_or(&'0'));
        }
        digits.into_iter().collect()
    }

    /// Implement a `special chars` generation
    fn gen_special(&mut self, len: usize) -> String {
        let mut spec: Vec<char> = Vec::with_capacity(len);

        for _ in 0..len {
            spec.push(*SPECIAL.choose(&mut self.rng).unwrap_or(&'!'));
        }
        spec.into_iter().collect()
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
                    Ok(UPWord(d)) => password.push(self.gen_pronounceable_word(*d, true)),
                    Ok(PWord(d)) => password.push(self.gen_pronounceable_word(*d, false)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_generates_requested_quantity() {
        let config = Config {
            format: vec![Ok(PassElements::Word(4)), Ok(PassElements::Digits(2))],
            quantity: 5,
            raw_format: "w4d2".to_string(),
        };

        let mut generator = Generator::new();
        let passwords = generator.run(config);

        assert_eq!(passwords.len(), 5);
    }

    #[test]
    fn run_generates_expected_password_length() {
        let config = Config {
            format: vec![
                Ok(PassElements::UWord(4)),
                Ok(PassElements::Special(2)),
                Ok(PassElements::PWord(5)),
                Ok(PassElements::Digits(3)),
            ],
            quantity: 1,
            raw_format: "W4s2p5d3".to_string(),
        };

        let mut generator = Generator::new();
        let passwords = generator.run(config);

        assert_eq!(passwords.len(), 1);
        assert_eq!(passwords[0].len(), 14);
    }
}
