pub fn run() {
    let answer = do_twice(add_one, 5);
    println!("answer: {}", answer);

    let answer_with_closure = do_twice(|a| {a + 1}, 5);
    println!("answer with closure: {}", answer_with_closure);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    for status in list_of_statuses.iter() {
        if let Status::Value(val) = status {
            print!("value: {}, ", val)
        }
    }
    print!("\n");

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
