# CLosure INTerrupt handlers

[![Documentation](https://docs.rs/clint/badge.svg)](https://docs.rs/clint)
[![Testing](https://api.travis-ci.org/repos/bjc/clint.svg?branch=master)](https://travis-ci.org/bjc/clint)

This crate allows you to use closures for interrupt handlers in a
heapless, no-std environment.

## Cargo features
The `HandlerTable` type uses a backing array for its closures. To
configure the number of available slots, specify one of the following
cargo features: `isr-8`, `isr-16`, `isr-32`, `isr-64`, `isr-128`, or
`isr-256`. By default, 32 slots are available.

# Example Code

See the `examples` directory for some simple examples.

For a slightly more complex example [this
repository](https://github.com/bjc/nrf52-demo.git) uses clint to blink
some LEDs and measure temperature across a number of interrupts and
exceptions.
