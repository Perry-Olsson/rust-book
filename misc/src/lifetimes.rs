use std::{io::Write};
use crate::terminal_io;

pub fn lifetimes() {
    let size: usize = terminal_io::get_user_input("Enter size: ").trim().parse().unwrap();
    let mut vec: Vec<i32> = Vec::with_capacity(size);
    vec.push(20);
    let my_values: &[i32] = vec.as_slice();
    for &val in my_values {
        print!("{},", val)
    }
    std::io::stdout().flush().unwrap();
}