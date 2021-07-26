# tsf

Rust bindings for [TinySoundFont](https://github.com/schellingb/TinySoundFont/). This is a software synthesizer for 
SoundFont2 sound bank files.

Essentially this library can render audio data in 32-bit float or 16-bit (short) int frames. It renders that audio using
a loaded soundfont and programmatically-driven note information (soundfont bank and preset, MIDI note number and 
velocity; and optionally understands MIDI channels). 

## Example

See the tests in `lib.rs` for example usage, but we defer mostly to the documentation/examples given in the 
TinySoundFont repository linked above (though these are in C).

## In progress

Only some functions in `tsf.h` have nice bindings here. It is a work in progress to add the rest. They shouldn't be too
difficult based on the ones that have already been done, so feel free to submit a PR with the ones you need! (Or, submit
an issue for the ones you need, and a maintainer will look at it if they have time.)

## Build machine requirements

You must install clang to build this crate, even if you are just using it as a dependency of another crate. This is necessary because the bindings are generated during the build step using
`bindgen` which requires an installation of clang. See https://rust-lang.github.io/rust-bindgen/requirements.html.

## Linking to the TinySoundFont library

TinySoundFont is distributed as a [header file](https://github.com/schellingb/TinySoundFont/blob/master/tsf.h). This crate includes a static library compiled from that header file, 
which makes it incredibly convenient to use - you don't need to do anything special.

If you are already using a library built from TinySoundFont and would like to link this crate to that library, this is currently
not possible. However, it's possible in theory, with a few feature flags, so feel free to submit a pull request (you will 
need to make changes to tsf-sys).
