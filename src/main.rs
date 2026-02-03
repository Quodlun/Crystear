use rand::prelude::*;
use text_io::read;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~";

enum CharType {
    Uppercase,
    Lowercase,
    Numbers,
    Symbols,
}

struct Characters {
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}

impl Characters {
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
}

fn main() {
    let mut characters_settings: Characters = Characters {
        uppercase: true,
        lowercase: true,
        numbers: true,
        symbols: true,
    };

    main_page_mode_select(&mut characters_settings);
}

fn main_page_mode_select(characters_settings: &mut Characters) {
    const GREETING_MESSAGE: &str = "Welcome to Crystear";
    let mut title_message = String::from(GREETING_MESSAGE);

    loop {
        clear_screen();
        println!("{}", title_message);
        println!("1. Generate Password");
        println!("2. Settings");
        println!("3. Exit");

        match read!() {
            1 => {
                match generate_password(characters_settings) {
                    Ok(_) => title_message = String::from(GREETING_MESSAGE),
                    Err(e) => title_message = e,
                }

                continue;
            }
            2 => {
                setting_mode_select(characters_settings);
                title_message = String::from(GREETING_MESSAGE);
            }
            3 => return,
            _ => (),
        }
    }
}

fn generate_password(characters_settings: &Characters) -> Result<(), String> {
    let pool = characters_settings.get_pool();
    let chars: Vec<char> = pool.chars().collect();

    if chars.is_empty() {
        return Err(String::from(
            "Warning! No Characters Selected! Please select characters in [Settings]",
        ));
    }

    let mut rng = rand::rng();

    loop {
        clear_screen();
        println!("Generate Password");

        print!("Password Length: ");
        let length_input: String = read!();

        match length_check(&length_input)
        {
            Ok(length) => {
                let mut result: String = String::with_capacity(length);

                for _ in 0..length {
                    result.push(chars[rng.random_range(0..chars.len())]);
                }

                println!("Result: {}", result);
            }

            Err ( e ) =>
            {
                println! ( "Someting went wrong: {}", e );
            }
        }

        println!("1. Re-generate Password");
        println!("2. Back");

        match read!() {
            1 => continue,
            2 => break,
            _ => (),
        }
    }

    Ok(())
}

fn setting_mode_select(characters_settings: &mut Characters) {
    loop {
        clear_screen();
        println!("Settings Menu:");
        println!("1. Using Characters");
        println!("2. Back");

        match read!() {
            1 => using_characters(characters_settings),
            2 => return,
            _ => (),
        }
    }
}

fn using_characters(characters_settings: &mut Characters) {
    let check_mark = |status: bool| if status { "V" } else { " " };

    loop {
        clear_screen();
        println!("Character Settings: (Select Numbers To Toggle)");
        println!(
            "1. [ {} ] Uppercase Letters",
            check_mark(characters_settings.uppercase)
        );
        println!(
            "2. [ {} ] Lowercase Letters",
            check_mark(characters_settings.lowercase)
        );
        println!("3. [ {} ] Numbers", check_mark(characters_settings.numbers));
        println!("4. [ {} ] Symbols", check_mark(characters_settings.symbols));
        println!("5. Back");

        match read!() {
            1 => characters_settings.toggle(CharType::Uppercase),
            2 => characters_settings.toggle(CharType::Lowercase),
            3 => characters_settings.toggle(CharType::Numbers),
            4 => characters_settings.toggle(CharType::Symbols),
            5 => return,
            _ => (),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn length_check ( input: &str ) -> Result < usize, String >
{
    let length: usize = input.trim ().parse::< usize > (). map_err (| _ | String :: from ( "Please enter a valid number." ) ) ? ;
    
    if length == 0 || length > 100
    {
        return Err ( String :: from ( "Length must be between 0 and 100." ) );
    }

    Ok ( length )
}