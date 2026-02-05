use rand::prelude::*;
// use std::string;

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~";

pub enum CharType {
    Uppercase,
    Lowercase,
    Numbers,
    Symbols,
}

pub struct GeneratorOption {
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

impl GeneratorOption {
    pub fn new(uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> Self {
        Self {
            uppercase,
            lowercase,
            numbers,
            symbols,
        }
    }

    pub fn toggle(&mut self, t: CharType) {
        let is_last_one = self.active_count_check();

        let field = match t {
            CharType::Uppercase => &mut self.uppercase,
            CharType::Lowercase => &mut self.lowercase,
            CharType::Numbers => &mut self.numbers,
            CharType::Symbols => &mut self.symbols,
        };

        if !*field || !is_last_one {
            *field = !*field;
        }
    }

    fn active_count_check(&self) -> bool {
        let active_count = [self.uppercase, self.lowercase, self.numbers, self.symbols]
            .iter()
            .filter(|&&active| active)
            .count();

        active_count == 1
    }

    pub fn get_pool(&self) -> String {
        let mut pool = String::new();

        if self.uppercase {
            pool.push_str(UPPERCASE);
        }

        if self.lowercase {
            pool.push_str(LOWERCASE);
        }

        if self.numbers {
            pool.push_str(NUMBERS);
        }
        if self.symbols {
            pool.push_str(SYMBOLS);
        }

        pool
    }

    pub fn generator(&self, chars: Vec<char>, length: usize) -> Result<String, String> {
        let mut rng = rand::rng();

        if chars.is_empty() {
            return Err(String::from("Warning! No Characters Selected! Please select characters in [Settings]"));
        }

        let result: String = (0..length)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars[idx]
            })
            .collect();

        Ok(result)
    }

    pub fn length_check(input: &str) -> Result<usize, String> {
        let length: usize = input
            .trim()
            .parse::<usize>()
            .map_err(|_| String::from("Please enter a valid number."))?;

        if length == 0 || length > 100 {
            return Err(String::from("Length must be between 1 and 100."));
        }

        Ok(length)
    }
}
