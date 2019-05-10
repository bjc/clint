//! Critical Section support.
//!
//! Types that implement the `CriticalSection` trait can be used to
//! create critical sections to prevent data races between interrupt
//! and non-interrupt contexts.
//!
//! # Note
//!
//! Critical sections are only acquired for updating a
//! `HandlerTable`'s entries. They are *not* used when calling into a
//! closure. This is because the expected implementation of critical
//! sections turns off interrupts entirely. Given that interrupts are
//! off, it is impossible to call an ISR, and thus no data race can
//! occur. Additionally, because critcal sections are only enforced on
//! updates, deadlock is impossible between updating a `HandlerTable`
//! entry and calling into it.
//!
//! However, if you are going to implement your own `CriticalSection`,
//! you need to be aware of this limitation and its rationale to avoid
//! getting into trouble.

/// Generic trait which supplies the ability to create a critical
/// section.
pub trait CriticalSection {
    /// Execute `f` within a critical section.
    fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R;
}

#[cfg_attr(any(all(target_arch = "arm", target_os = "none")), path = "cortex.rs")]
#[cfg_attr(
    not(any(all(target_arch = "arm", target_os = "none"))),
    path = "dummy.rs"
)]
mod csimpl;

pub use csimpl::Locker;
