use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(Rc::new(RefCell::new(1)), Rc::new(Nil)));
    let b = Cons(
        Rc::new(RefCell::new(10)),
        Rc::new(Cons(Rc::new(RefCell::new(11)), Rc::clone(&a))),
    );
    let c = Cons(Rc::new(RefCell::new(100)), Rc::clone(&a));

    // if let Cons(v, _) = &*a {
    if let Cons(ref v, _) = *a {
        *v.borrow_mut() = 5;
    }

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
