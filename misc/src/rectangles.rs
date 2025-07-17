use crate::terminal_io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn calculate_rectangle_area() {
    let outer_rectangle = Rectangle {
        width: 100,
        height: 200
    };

    let rectangle = Rectangle {
        height: terminal_io::get_user_input("Enter height:").trim().parse().unwrap(),
        width: terminal_io::get_user_input("Enter width:").trim().parse().unwrap(),
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rectangle.area()
    );

    if outer_rectangle.can_hold(&rectangle) {
        println!("Rectangle is valid")
    } else {
        println!("Rectangle is too large")
    }

    let square = Rectangle::square(40);

    println!("Square: {square:?}")
}
