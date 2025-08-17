pub fn run() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    while_let();
    print_coordinates(&(123, 321));
    axis(&Point { x: 12, y: 1 })
}

fn while_let() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(val) = rx.recv() {
        println!("value: {}", val);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("{{x: {}, y: {}}}", x, y);
}

struct Point { x: i32, y: i32 }

fn axis(point: &Point) {
    match point {
        Point {x, y: 0} => println!("on y axis at {x}"),
        Point {x: 0, y} => println!("on x axis at {y}"),
        Point {x, y} => println!("On neither axis. x: {x}, y: {y}")
    }
}
