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
        print!("\x1B[2J");
        println! ( "Welcome To Crystear." );
        println! ( "1. Generate Password" );
        println! ( "2. Settings" );
        println! ( "3. Exit" );

        match read! ()
        {
            1 => print!("\x1B[2J"),
            2 => setting_mode_select ( characters_settings ),
            3 => std::process::exit(0),
            _ => ()
        }
    }
}

fn setting_mode_select ( characters_settings: &mut Characters )
{
    loop
    {
        print!("\x1B[2J");
        println! ( "Settings Menu:" );
        println! ( "1. Using Characters" );
        println! ( "2. Back" );

        match read! ()
        {
            1 => using_characters ( characters_settings ),
            2 => main_page_mode_select ( characters_settings ),
            _ => ()
        }
    }
}

fn using_characters ( characters_settings: &mut Characters )
{
    loop
    {
        print!("\x1B[2J");
        println! ( "Character Settings: (Select Numbers To Change " );
        println! ( "1. Use Uppercase Letters: {}", characters_settings.uppercase );
        println! ( "2. Use Lowercase Letters: {}", characters_settings.lowercase );
        println! ( "3. Use Numbers: {}", characters_settings.numbers );
        println! ( "4. Use Symbols: {}", characters_settings.symbols );
        println! ( "5. Back" );

        match  read! ()
        {
            1 => characters_settings.uppercase = !characters_settings.uppercase,
            2 => characters_settings.lowercase = !characters_settings.lowercase,
            3 => characters_settings.numbers = !characters_settings.numbers,
            4 => characters_settings.symbols = !characters_settings.symbols,
            5 => { setting_mode_select ( characters_settings ); break; },
            _ => (),
        }
    }
}

