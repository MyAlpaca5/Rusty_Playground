use std::{sync::mpsc, thread};

fn main() {
    let nums = vec![1, 2, 3];
    let (tx, rx) = mpsc::channel();

    for _ in 1..4 {
        let producer = tx.clone();
        // other way is to use scoped thread, https://docs.rs/crossbeam/0.8.1/crossbeam/thread/index.html
        let nums_local = nums.clone();
        thread::spawn(move || {
            for num in nums_local {
                producer.send(num).unwrap();
            }
        });
    }
    // need to close tx itself, otherwise, rx cannot close.
    drop(tx);

    for rec in rx {
        println!("{}", rec);
    }
}
