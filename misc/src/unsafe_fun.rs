use std::slice;

pub fn run() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    slice();
    call_abs_from_c_std_lib();
    println!("{}", HELLO_WORLD);
    unsafe {
        // SAFETY: This is only called from the main thread
        add_to_count(123);
        println!("Counter: {}", *(&raw const COUNTER));
    }
}

unsafe trait Foo {
    fn some_method();
}

unsafe impl Foo for i32 {
    fn some_method() {
        print!("i32 implemntation")
    }
}

static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: i32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn call_abs_from_c_std_lib() {
    println!("absolute value of -3 according to c: {}", abs(-3));
}

unsafe extern "C" {
    safe fn abs(val: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from c")
}

fn slice() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut_impl(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

}

fn split_at_mut_impl(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid < len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

unsafe fn invalid_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("{}", *r);
    }
}
