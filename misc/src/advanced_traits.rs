use std::ops::Add;
use std::fmt;

pub fn run() {
    let point = Point { x: 2, y: 3 };
    assert_eq!(
        Point { x: 1, y: 0 } + point,
        Point { x: 3, y: 3 }
    );

    let person = Human{};
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);
    point.outline_print();
    let w = Strings(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(i32);
struct Meters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Wizard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Captain")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Harry Potter")
    }
}

impl Human {
    fn fly(&self) {
        println!("I can't fly")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {
}

struct Strings(Vec<String>);

impl fmt::Display for Strings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", self.0.join(", "))
    }
}
