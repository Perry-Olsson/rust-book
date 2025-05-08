pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Move{ x, y } => println!("Is Move"),
            Self::Quit => println!("I quit"),
            _ => println!("Not move")
        }
    }
}

pub fn run() {
    let message = Message::Move{x:123, y:321};
    let other_message = Message::Quit;

    message.call();
    other_message.call()
}
