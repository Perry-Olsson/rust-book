use crate::terminal_io;

pub fn farenheit_to_celsius() {
    let my_str = String::from("Enter degrees in farenheit: ");
    let degrees_farenheit: f64 = loop {
        match terminal_io::get_user_input(&my_str).trim().parse() {
            Ok(n) => break n,
            Err(e) => println!("{}. Try again", e)
        };
    };
    println!("{} degrees in celsius", convert_farenheit_to_celsius(degrees_farenheit));
}

fn convert_farenheit_to_celsius(deg: f64) -> f64 {
    return (deg - 32.0) * 5.0 / 9.0
}