# CLosure INTerrupt handlers

This crate allows you to use closures for interrupt handlers in a
heapless, no-std environment.

## Cargo features
The `HandlerTable` type uses a backing array for its closures. To
configure the number of available slots, specify one of the following
cargo features: `isr-8`, `isr-16`, `isr-32`, `isr-64`, `isr-128`, or
`isr-256`. By default, 32 slots are available.

If you're using a nightly toolchain, you can enable an optimization
when calling handlers by turning on the `const-fn` feature. This will
save one conditional branch on every call.

# Example Code

See the `examples` directory for some simple examples.

For a slightly more complex example [this
repository](https://github.com/bjc/nrf52-demo.git) uses clint to blink
some LEDs and measure temperature across a number of interrupts and
exceptions.
