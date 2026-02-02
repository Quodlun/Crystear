use text_io::read;
use rand::prelude::*;

enum CharType
{
    Uppercase,
    Lowercase,
    Numbers,
    Symbols,
}

struct Characters
{
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}

impl Characters
{
    pub fn toggle ( &mut self, t: CharType )
    {
        let is_last_one = self.active_count_check();

        let field = match t
        {
            CharType::Uppercase => &mut self.uppercase,
            CharType::Lowercase => &mut self.lowercase,
            CharType::Numbers => &mut self.numbers,
            CharType::Symbols => &mut self.symbols,
        };

        if !*field || !is_last_one
        {
            *field = !*field;
        }
    }

    fn active_count_check ( &self ) -> bool
    {
        let active_count = [ self.uppercase, self.lowercase, self.numbers, self.symbols ]
            .iter()
            .filter( | &&active | active )
            .count();   

        active_count == 1
    }

    pub fn get_pool ( &self ) -> String
    {
        let mut pool = String::new();

        if self.uppercase { pool.push_str ( "ABCDEFGHIJKLMNOPQRSTUVWXYZ" ); }

        if self.lowercase { pool.push_str ( "abcdefghijklmnopqrstuvwxyz" ); }

        if self.numbers { pool.push_str ( "0123456789" ); }

        if self.symbols { pool.push_str ( "!@#$%^&*()-_=+[]{}|;:',.<>?/`~" ); }

        pool
    }
}

fn main ()
{
    let mut characters_settings: Characters = Characters
    {
        uppercase: true,
        lowercase: true,
        numbers: true,
        symbols: true,
    };

    main_page_mode_select ( &mut characters_settings );
}

fn main_page_mode_select ( characters_settings: &mut Characters )
{
    loop
    {
        clear_screen ();
        println! ( "Welcome To Crystear." );
        println! ( "1. Generate Password" );
        println! ( "2. Settings" );
        println! ( "3. Exit" );

        match read! ()
        {
            1 => generate_password ( characters_settings ),
            2 => setting_mode_select ( characters_settings ),
            3 => return,
            _ => ()
        }
    }
}

fn generate_password ( characters_settings: &Characters )
{
    let mut rng = rand::rng ();
    print! ( "\x1B[2J\x1B[H" );
    println! ( "Generate Password" );

    print! ( "Password Length: " );
    let length_input:String = read! ();
    let length: usize = length_input.trim().parse().unwrap_or ( 0 );
    
    if length == 0
    {
        println! ( "Invalid Length!" );
    }
    
    else
    {
        let pool = characters_settings.get_pool ();
        let chars: Vec<char> = pool.chars ().collect ();
        
        if chars.is_empty ()
        {
            println! ( "[ERROR] No Characters Selected!" );
            return;
        }

        else
        {
            let mut result: String = String::new ();

            for _ in 0 .. length
            {
                result.push ( chars [ rng.random_range ( 0 .. chars.len () ) ] );
            }

            println! ( "Result: {}", result );
        }
    }

    println! ( "1. Re-generate Password" );
    println! ( "2. Back" );

    match read! ()
    {
        1 => generate_password ( characters_settings ),
        2 => return,
        _ => ()
    }
}

fn setting_mode_select ( characters_settings: &mut Characters )
{
    loop
    {
        clear_screen ();
        println! ( "Settings Menu:" );
        println! ( "1. Using Characters" );
        println! ( "2. Back" );

        match read! ()
        {
            1 => using_characters ( characters_settings ),
            2 => return,
            _ => ()
        }
    }
}

fn using_characters ( characters_settings: &mut Characters )
{
    loop
    {
        clear_screen ();
        println! ( "Character Settings: (Select Numbers To Change " );
        println!("Character Settings: (Select Numbers To Toggle)");
        println!("1. [ {} ] Uppercase Letters", if characters_settings.uppercase { "V" } else { " " });
        println!("2. [ {} ] Lowercase Letters", if characters_settings.lowercase { "V" } else { " " });
        println!("3. [ {} ] Numbers", if characters_settings.numbers { "V" } else { " " });
        println!("4. [ {} ] Symbols", if characters_settings.symbols { "V" } else { " " });
        println! ( "5. Back" );

        match  read! ()
        {
            1 => characters_settings.toggle ( CharType::Uppercase ),
            2 => characters_settings.toggle ( CharType::Lowercase ),
            3 => characters_settings.toggle ( CharType::Numbers ),
            4 => characters_settings.toggle ( CharType::Symbols ),
            5 => return,
            _ => (),
        }
    }
}

fn clear_screen ()
{
    print!("\x1B[2J\x1B[H");
}