extern crate clint;

#[macro_use]
extern crate lazy_static;

use clint::Handler;

lazy_static! {
    static ref HANDLER: Handler<'static> = Handler::new();
}

fn main() {
    need_move();
    borrow_error();
    no_borrow_needed();
}

fn need_move() {
    let x = vec![1, 2, 3];
    let mut c = || {
        println!("x(h-c): {:?}", x); //~ ERROR does not live long enough
    };
    unsafe {
        HANDLER.replace(&mut c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x);
}

fn borrow_error() {
    let x = vec![1, 2, 3];
    let mut c = move || {
        println!("x(h-c): {:?}", x);
    };
    unsafe {
        HANDLER.replace(&mut c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x); //~ ERROR borrow of moved value
}

fn no_borrow_needed() {
    let x = vec![1, 2, 3];
    let mut c = || {
        println!("x(h-c): hi!");
    };
    unsafe {
        HANDLER.replace(&mut c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x);
}
