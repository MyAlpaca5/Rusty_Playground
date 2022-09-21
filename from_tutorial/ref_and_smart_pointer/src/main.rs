#![allow(dead_code)]

fn main() {
    let x = 55;
    let mut y = 55;

    let _a = &x;
    let _b = &mut y;
    //  ===
    let ref _aa = x;
    let ref mut _bb = y;

    let z = Some("xxx".to_owned());
    match z {
        Some(ref val) => println!("this is reference value {}", val),
        None => println!("no value"),
    }

    println!("still can use z here {:?}", z);

    let zz = Some("xxxx".to_owned());
    match zz {
        Some(val) => println!("this is consumed value {}", val),
        None => println!("no value"),
    }
}
