use std::io::Write;

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut buf = String::new();
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut buf) {
        Ok(input) => input,
        Err(e) => panic!("Problem processing input. Error: {}", e)
    };
    buf
}