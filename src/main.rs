use text_io::read;

struct Characters
{
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
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
        print!("\x1B[2J\x1B[H");
        println! ( "Welcome To Crystear." );
        println! ( "1. Generate Password" );
        println! ( "2. Settings" );
        println! ( "3. Exit" );

        match read! ()
        {
            1 => print!("\x1B[2J"),
            2 => setting_mode_select ( characters_settings ),
            3 => return,
            _ => ()
        }
    }
}

fn setting_mode_select ( characters_settings: &mut Characters )
{
    loop
    {
        print!("\x1B[2J\x1B[H");
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
        print!("\x1B[2J\x1B[H");
        println! ( "Character Settings: (Select Numbers To Change " );
        println!("Character Settings: (Select Numbers To Toggle)");
        println!("1. [ {} ] Uppercase Letters", if characters_settings.uppercase { "V" } else { " " });
        println!("2. [ {} ] Lowercase Letters", if characters_settings.lowercase { "V" } else { " " });
        println!("3. [ {} ] Numbers", if characters_settings.numbers { "V" } else { " " });
        println!("4. [ {} ] Symbols", if characters_settings.symbols { "V" } else { " " });
        println! ( "5. Back" );

        if characters_settings.uppercase || characters_settings.lowercase || characters_settings.numbers || characters_settings.symbols
        {
            match  read! ()
            {
                1 => characters_settings.uppercase = !characters_settings.uppercase,
                2 => characters_settings.lowercase = !characters_settings.lowercase,
                3 => characters_settings.numbers = !characters_settings.numbers,
                4 => characters_settings.symbols = !characters_settings.symbols,
                5 => return,
                _ => (),
            }
        }
    }
}