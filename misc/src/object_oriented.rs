use std::fmt::{Display, Write};

pub fn run() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.draw();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn draw(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button(width: {}, height: {}, label: {})", self.width, self.height, self.label)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{}", self);
    }
}

impl Display for SelectBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectBox(width: {}, height: {}, options: ", self.width, self.height)?;
        f.write_char('[')?;
        for (i, option) in self.options.iter().enumerate() {
            if i == self.options.len() - 1 {
                f.write_str(option)?;
                f.write_char(']')?;
            } else {
                f.write_str(option)?;
                f.write_str(", ")?;
            }
        }
        f.write_char(')')
    }
}
