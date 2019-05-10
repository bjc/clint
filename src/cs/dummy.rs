use super::CriticalSection;

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
        f()
    }
}
