use crate::terminal_io;

enum CharType {
    Vowel,
    Digit,
    Consonant
}

pub fn convert_to_pig_latin() {
    let sentance = terminal_io::get_user_input("Type your sentence: ");
    let mut pig_latin = String::with_capacity(sentance.len() * 1.5 as usize);
    for word in sentance.split_whitespace() {
        let first_char = word.chars().nth(0).expect("Unexpected empty word");
        match get_char_type(first_char) {
            CharType::Vowel => handle_vowel(&mut pig_latin, word),
            CharType::Consonant => {
                let rest_of_word = &word[1..];
                pig_latin.push_str(rest_of_word);
                pig_latin.push(first_char);
                pig_latin.push_str("ay ");
            }
            CharType::Digit => {
                pig_latin.push_str(word);
                pig_latin.push(' ');
            }
        }
    }

    println!("{}", pig_latin)
}

fn get_char_type(char: char) -> CharType {
    if char.is_ascii_digit() {
        return CharType::Digit
    }

    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => CharType::Vowel,
        _ => CharType::Consonant 
    }
}

fn handle_vowel(pig_latin: &mut String, word: &str) {
    pig_latin.push_str(word);
    pig_latin.push_str("hay ");
}