#![feature(test)]

extern crate test;
use test::Bencher;

use clint::Handler;

const ITER_COUNT: usize = 10_000;

#[bench]
fn bench_bare_fn(b: &mut Bencher) {
    static mut X: usize = 0;
    #[inline(never)]
    fn inc() {
        unsafe { X += 1 };
    }

    let n = test::black_box(ITER_COUNT);
    b.iter(|| (0..n).for_each(|_| inc()));
    assert!(unsafe { X } > 0);
}

#[bench]
fn bench_handler(b: &mut Bencher) {
    static mut X: usize = 0;
    let mut handler = Handler::new();
    unsafe { handler.replace(&move || X += 1) };

    let n = test::black_box(ITER_COUNT);
    b.iter(|| (0..n).for_each(|_| unsafe { handler.call() }));
    assert!(unsafe { X } > 0);
}
