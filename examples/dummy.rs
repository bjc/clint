use clint::Handler;

// Wrapper used to call through to `example_handler` via `closure` in
// `main`. `Handler::new()` places a do-nothing handler in this at
// compile-time, in case the interrupt using this handler is fired
// before being `replace`d in `main`.
static mut HANDLER: Handler = Handler::new();

fn main() {
    let mut x: u32 = 0;

    // Create a closure to take a mutable reference to `x` for use in
    // `example_handler`.
    let closure = move || example_handler(&mut x);

    // Swap out the do-nothing handler with our closure that calls
    // through to `example_handler`. Ideally, the interrupt which uses
    // this handler would be disabled while this happens, but as this
    // is a demo, and there aren't any actual interrupts firing, this
    // is left as an exercise to the reader.
    unsafe { HANDLER.replace(&closure) };

    // Simulate firing the interrupt.
    dummy_interrupt();
    dummy_interrupt();

    // Because `x` is `Copy`, we still have access to the symbol,
    // although its value won't be changed by `closure`.
    println!("x(o): {}", x);
}

// Not a real interrupt handler, but called like one. i.e.: simple
// function with no arguments.
//
// Calls through `HANDLER` to do its actual work.
fn dummy_interrupt() {
    unsafe { HANDLER.call() };
}

// The meat of the interrupt handler, which does work with whatever
// params were passed in via `closure` in `main`.
fn example_handler(x: &mut u32) {
    // Update our dummy value, just to show it works.
    *x += 2;
    println!("x: {}", x);
}
