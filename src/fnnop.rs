// This module needs the following features.
//
//#![feature(unboxed_closures)]
//#![feature(fn_traits)]
use core::cell::UnsafeCell;

pub struct FnNOP();

impl Fn<()> for FnNOP {
    extern "rust-call" fn call(&self, _args: ()) {}
}
impl FnMut<()> for FnNOP {
    extern "rust-call" fn call_mut(&mut self, _args: ()) {}
}
impl FnOnce<()> for FnNOP {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: ()) {}
}

pub static mut NOP: UnsafeCell<FnNOP> = UnsafeCell::new(FnNOP());
