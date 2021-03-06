//! CLosure INTerrupt handlers
//!
//! Use closures as interrupt service routines to leverage Rust's
//! borrow checker for safe, exclusive usage of device peripherals and
//! other data without locking.
//!
//! # Layout
//!
//! See [`array`'s module documentation](table/index.html#examples) for basic,
//! safe usage.
//!
//! The [`handler`](handler) module contains the underyling, unsafe
//! implementation.
//!
//! Critical section support is supplied by the [`cs` module](cs).

#![no_std]
#![feature(const_fn)]

pub mod array;
pub mod cs;
pub mod handler;

pub use array::HandlerArray;
pub use handler::Handler;
