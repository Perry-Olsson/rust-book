use std::fmt::{Debug};

fn main() {
    test("hello")
}

fn test<T: Debug>(par: T) {
    println!("something: {:?}", par)
}