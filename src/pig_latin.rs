use crate::terminal_io;

pub fn convert_to_pig_latin() {
    let sentance = terminal_io::get_user_input("Type your sentence: ");
    let mut pig_latin = String::with_capacity(sentance.len() * 1.5 as usize);
    for word in sentance.split_whitespace() {
        let first_char = word.chars().nth(0).unwrap();
        if is_vowel(first_char) {
            handle_vowel(&mut pig_latin, word);
        } else {
            let rest_of_word = &word[1..];
            pig_latin.push_str(rest_of_word);
            pig_latin.push(first_char);
            pig_latin.push_str("ay ");
        }
    }

    println!("{}", pig_latin)
}

fn is_vowel(char: char) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn handle_vowel(pig_latin: &mut String, word: &str) {
    pig_latin.push_str(word);
    pig_latin.push_str("hay ");
}