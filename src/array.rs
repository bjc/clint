//! Safe wrapper for closure-based interrupt handlers.
//!
//! # Notes
//!
//! The number of entries allowed is defined by Cargo features. The
//! default is 32 as this seems a reasonable comprimise between the
//! size of the array and utility. Each array entry costs two words of
//! space for the closure reference. Thus a full array of 256 entries
//! on a 32-bit architecture costs 2048 bytes of memory, which can be
//! quite a lot on resource constrained devices.
//!
//! One day, when const-generics are stabilized, this will be more
//! elegant.
//!
//! # Examples
//!
//! ``` no_run
//! use clint::HandlerArray;
//! use cortex_m_rt::exception;
//!
//! #[macro_use]
//! extern crate lazy_static;
//!
//! lazy_static! {
//!     static ref HANDLERS: HandlerArray<'static> = HandlerArray::new();
//! }
//!
//! fn main() {
//!     // NB: This closure has to be created outside of `with_overrides` to
//!     // ensure it lives as long as `with_overrides` scope lasts.
//!     let mut cl = || {
//!         // Your interrupt handling code.
//!     };
//!     HANDLERS.with_overrides(|arr| {
//!         arr.register(0, &mut cl);
//!
//!         loop {
//!             // Your main loop.
//!         }
//!     })
//! }
//!
//! #[exception]
//! fn SysTick() {
//!     HANDLERS.call(0);
//! }
//! ```

use crate::cs::{CriticalSection, Locker};
use crate::Handler;

use core::cell::UnsafeCell;

// Define features for the underlying array size so that we can
// statically allocate it.
// TODO: Use const generics when available.
#[cfg(feature = "isr-8")]
const NR_ISR: usize = 8;
#[cfg(feature = "isr-16")]
const NR_ISR: usize = 16;
#[cfg(feature = "isr-32")]
const NR_ISR: usize = 32;
#[cfg(feature = "isr-64")]
const NR_ISR: usize = 64;
#[cfg(feature = "isr-128")]
const NR_ISR: usize = 128;
#[cfg(feature = "isr-256")]
const NR_ISR: usize = 256;

/// Safely use `Handler`s by enclosing them in an array.
///
/// This type provides a safe wrapper around `Handler` by ensuring
/// that closures are swapped safely using critical sections, and that
/// the lifetime of those handlers is sufficient by using the inner
/// scope of `with_overrides`/`lock_overrides`.
#[derive(Debug)]
pub struct HandlerArray<'a> {
    h: UnsafeCell<[Handler<'a>; NR_ISR]>,
}

impl<'a> HandlerArray<'a> {
    /// Create a new `HandlerArray` filled with no-op handlers.
    #[cfg(feature = "const-fn")]
    pub const fn new() -> Self {
        Self {
            h: UnsafeCell::new([Handler::new(); NR_ISR]),
        }
    }

    #[cfg(not(feature = "const-fn"))]
    pub fn new() -> Self {
        let h = {
            let mut ui_h: [core::mem::MaybeUninit<Handler>; NR_ISR] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for h in &mut ui_h[..] {
                unsafe { core::ptr::write(h.as_mut_ptr(), Handler::new()) }
            }
            unsafe { core::mem::transmute(ui_h) }
        };
        Self {
            h: UnsafeCell::new(h),
        }
    }

    /// Register `f` for entry `nr` in this array using the default
    /// critical section locker.
    pub fn register<F>(&self, nr: usize, f: &'a mut F)
    where
        F: FnMut() + Send + 'a,
    {
        self.lock_register(&Locker::new(), nr, f)
    }

    /// Register `f` for entry `nr` in this array using `cs` to create
    /// a critical section for updating the array.
    pub fn lock_register<F, CS>(&self, cs: &CS, nr: usize, f: &'a mut F)
    where
        F: FnMut() + Send + 'a,
        CS: CriticalSection,
    {
        cs.with_lock(|| unsafe { (*self.h.get())[nr].replace(f) });
    }

    /// Call the handler for entry `nr`.
    pub fn call(&self, nr: usize) {
        // Unsafe: there's always a valid handler to call except for
        // when it's being actively replaced. As long as that happens
        // while in a critical section, there's no risk of data races.
        unsafe { (*self.h.get())[nr].call() }
    }

    /// Create a new array for use in `f`'s scope. The existing
    /// handlers can be overridden using `register` or
    /// `lock_register`. When `f` exits, all previous handlers are
    /// restored.
    pub fn with_overrides<'b>(&self, f: impl FnOnce(&HandlerArray<'b>)) {
        self.lock_overrides(&Locker::new(), f)
    }

    /// Same as `with_overrides` but allows you to specify your own
    /// implementation of `CriticalSection` instead of using the
    /// default.
    pub fn lock_overrides<'b, CS>(&self, cs: &CS, f: impl FnOnce(&HandlerArray<'b>))
    where
        CS: CriticalSection,
    {
        // Create a shorter-lived array from `self` that matches the
        // lifetime of `f` so we can make sure `register` is only
        // called with closures that will live as long as `f` does.
        //
        // Unsafe: This requires that we back up and restore the handlers
        // in the array to make sure there's always something alive in
        // whatever the real scope of `array' is.
        let tmp: &HandlerArray<'b> = unsafe { core::mem::transmute(self) };

        // Back up old handlers before entering inner scope so we can
        // restore them on exit.
        let bk = HandlerArray::new();
        unsafe { core::ptr::copy_nonoverlapping(tmp.h.get(), bk.h.get(), 1) }
        f(tmp);

        // Put the old handlers back inside a critical section to avoid
        // data races.
        cs.with_lock(|| unsafe { core::ptr::copy_nonoverlapping(bk.h.get(), tmp.h.get(), 1) });
    }
}

// Unsafe: as long as `register` and `with_overrides` use critical
// sections appropriately, it should be safe to share this between
// threads.
unsafe impl<'a> Sync for HandlerArray<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn overrides_unwind() {
        static mut CALLS: usize = 0;
        let mut cl = || unsafe { CALLS += 1 };
        let cl_ref = &mut cl;

        let ht = HandlerArray::new();
        ht.with_overrides(|t| {
            t.register(0, cl_ref);
            ht.call(0);
        });
        unsafe { assert_eq!(CALLS, 1) };
        ht.call(0);
        unsafe { assert_eq!(CALLS, 1) };
    }
}
