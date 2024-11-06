# Vuquest 3320

This crate supports basic communication with Vuquest 3320[g] barcode scanner devices.

Included are data structures and algorithms for the serial communication protocol to control and configure the barcode scanner.

## no-std support

`vuquest-3320` supports `no-std` by default, but currently requires `alloc`. Future versions may include a `no-alloc` subset of the library.

`std`-only capabilities can be enabled using the `std` feature.
