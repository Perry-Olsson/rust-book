use List::{Cons, Nil};
use std::fmt;
use std::ops::{Deref};
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn print_con(&self, list: &List, buf: &mut String) {
        match list {
            Cons(n, c) => {
                buf.push('(');
                buf.push_str(n.to_string().as_str());
                match **c {
                    Cons(_, _) => { buf.push_str(", ")},
                    Nil => {}
                }
                self.print_con(c, buf);
                buf.push(')');
            },
            Nil => {}
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let list = self;
        let mut result = String::new();
        self.print_con(list, &mut result);
        write!(f, "{}", result)
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn run() {
    let list_a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("count after creating a = {}", Rc::strong_count(&list_a));
    let list_b = Cons(2, Rc::clone(&list_a));
    println!("count after creating b = {}", Rc::strong_count(&list_a));
    {
        let _list_c = Cons(3, Rc::clone(&list_a));
        println!("count after creating c = {}", Rc::strong_count(&list_a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&list_a));


    println!("list b: {}", list_b);

    let x = 5;
    let y = MyBox::new(x);

    println!("x: {}", x);
    println!("y: {}", *y);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
