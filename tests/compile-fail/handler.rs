extern crate clint;

use clint::Handler;

static mut HANDLER: Handler = Handler::new();

fn main() {
    need_move();
    borrow_error();
    no_borrow_needed();
}

fn need_move() {
    let x = vec![1, 2, 3];
    let c = || {
        println!("x(h-c): {:?}", x); //~ ERROR does not live long enough
    };
    unsafe {
        HANDLER.replace(&c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x);
}

fn borrow_error() {
    let x = vec![1, 2, 3];
    let c = move || {
        println!("x(h-c): {:?}", x);
    };
    unsafe {
        HANDLER.replace(&c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x); //~ ERROR borrow of moved value
}

fn no_borrow_needed() {
    let x = vec![1, 2, 3];
    let c = || {
        println!("x(h-c): hi!");
    };
    unsafe {
        HANDLER.replace(&c);
        HANDLER.call();
        HANDLER.call();
    }
    println!("x(h-o): {:?}", x);
}
