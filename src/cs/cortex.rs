use super::CriticalSection;

use cortex_m::interrupt;

pub struct Locker {}

impl Locker {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CriticalSection for Locker {
    fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        interrupt::free(|_cs| f())
    }
}
