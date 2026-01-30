use text_io::read;

fn main ()
{
    println! ( "Welcome To Crystear." );
    
    print! ( "Password Length: ");
    let password_length: i32 = read! ();

    print! ( "Length Set To: {}\n", password_length );
}