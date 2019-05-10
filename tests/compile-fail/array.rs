extern crate clint;

use clint::cs::Locker;
use clint::HandlerArray;

fn main() {
    let mut hs = HandlerArray::new();
    hs.with_overrides(|new_hs| nested(new_hs));
}

fn nested(hs: &HandlerArray) {
    let mut c = || println!("Short-lived closure.");
    hs.register(0, &mut c) //~ ERROR `c` does not live long enough
}
