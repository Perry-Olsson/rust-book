struct Num {
    x: i32
}

fn main() {
    let num = Num { x: 20 };
    print_val(&num);
    println!("{}", num.x * 2)
}

fn print_val(num: &Num) {
    println!("{}", num.x)
}
