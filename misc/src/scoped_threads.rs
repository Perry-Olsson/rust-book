use std::{thread, time::Duration};
use std::time::Instant;

pub fn run() {
    let a = vec![1, 2, 3];
    let mut x = 0;

    let start = Instant::now();

    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3));
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3));
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
    });
    println!("Executing scoped threads took {} seconds", start.elapsed().as_secs());
}
