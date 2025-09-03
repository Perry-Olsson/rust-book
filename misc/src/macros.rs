use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Syrup {
    viscosity: u8
}

pub fn run() {
    Pancakes::hello_macro();
    Syrup::hello_macro();
}

/* #[macro_export]
macro_rules! simple_vec {
    ( $( $x:expr ),* ) => {
        let x = 5;
        $(
            x += $x
        )*
    }
} */
