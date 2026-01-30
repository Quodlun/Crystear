use text_io::read;
fn mode_select () -> i32
{
    println! ( "1. Generate Password" );
    println! ( "2. Settings" );
    println! ( "3. Exit" );

    let mode: i32 = read! ();

    return mode;
}

fn mode_switch ( mode: i32 )
{
    match mode
    {
        1 => print!("\x1B[2J"),
        2 => println! ( "Opening Settings..." ),
        3 => println! ( "Exiting..." ),
        _ => println! ( "Invalid Option Selected." ),
    }
}

/*
fn length_setting () -> i32
{
    

    print! ( "Password Length: ");
    let password_length: i32 = read! ();

    return password_length;
}
*/



fn main ()
{
    println! ( "Welcome To Crystear." );

    mode_switch ( mode_select () );
    

    // let password_length: i32 = length_setting ();

    // print! ( "Length Set To: {}\n", password_length );
}