# tsf-sys

Unsafe rust bindings for [TinySoundFont](https://github.com/schellingb/TinySoundFont/). Don't use this crate directly, 
instead use `tsf` which has more user-friendly bindings.

## Contributing

This crate was written by following 
[this rust-bindgen tutorial](https://rust-lang.github.io/rust-bindgen/library-usage.html). If you can follow that 
tutorial, then you understand how this crate works.

## Linkage to the TinySoundFont library

TinySoundFont is distributed as a header file. This crate includes a static library compiled from that header file,
which makes it incredibly convenient to use - you don't need to do anything special.

If you are already using a library built from TinySoundFont and would like to link this crate library, this is currently
not possible. However, it's possible in theory with a few feature flags, so feel free to submit a pull request (you will
need to make changes to tsf-sys).