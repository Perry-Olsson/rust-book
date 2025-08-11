pub fn run() {
    let ob1 = ObjOne::new();
    let ob2 = ObjTwo::new();
    let traits: Vec<&dyn MyTrait> = vec![&ob1, &ob2];
    execute(traits);
}

fn execute(my_traits: Vec<&dyn MyTrait>) {
    for t in my_traits {
        t.do_something()
    }
}

trait MyTrait {
    fn do_something(&self);
}

struct ObjOne {
    some_val: &'static str
}

impl ObjOne {
    fn new() -> ObjOne {
        ObjOne { some_val: "hello" }
    }
}

struct ObjTwo {
    some_val: &'static str
}

impl ObjTwo {
    fn new() -> ObjTwo {
        ObjTwo { some_val: "world" }
    }
}

impl MyTrait for ObjOne {
    fn do_something(&self) {
        println!("{}", &self.some_val);
    }
}

impl MyTrait for ObjTwo {
    fn do_something(&self) {
        println!("{}", &self.some_val);
    }
}
