#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node {
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    val: i32,
}

fn main() {
    let leaf = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
        val: 111,
    });

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            val: 999,
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "leaf strong: {}, weak: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!(
            "branch strong: {}, weak: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
