pub enum Message<'a> {
    Quit,
    Move { x: i32, y: i32 },
    Write(&'a str),
    ChangeColor(i32, i32, i32),
}

impl Message<'_> {
    fn call(&self) {
        match self {
            Self::Move{ x, y } => println!("Move to x: {}, y: {}", x, y),
            Self::Quit => println!("I quit"),
            Self::Write(val) => println!("{}", val),
            Self::ChangeColor(val1, val2, val3) => println!("values: ({}, {}, {})", val1, val2, val3)
        }
    }
}

pub fn run() {
    let mut message = Message::Move{x:123, y:321};
    message.call();
    message = Message::Quit;
    message.call();
    message = Message::Write("Hello, world");
    message.call();
    message = Message::ChangeColor(5, 6, 7);
    message.call();
}
