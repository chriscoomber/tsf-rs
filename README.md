# rust bindings for [TinySoundFont](https://github.com/schellingb/TinySoundFont/) &emsp; [![Latest Version]][crates.io] 

[Latest Version]: https://img.shields.io/crates/v/tsf.svg
[crates.io]: https://crates.io/crates/tsf

TinySoundFont is a software synthesizer for SoundFont2 sound bank files.

Essentially it can render audio data in 32-bit float or 16-bit (short) int frames. It renders that audio using
a loaded soundfont and programmatically-driven note information (soundfont bank and preset, MIDI note number and 
velocity; and optionally understands MIDI channels). 

## Example

See the tests in `lib.rs` for example usage, but we defer mostly to the documentation/examples given in the 
TinySoundFont repository linked above (though these are in C).

## In progress

Only some functions in `tsf.h` have nice bindings here. It is a work in progress to add the rest. They shouldn't be too
difficult based on the ones that have already been done, so feel free to submit a PR with the ones you need! (Or, submit
an issue for the ones you need, and a maintainer will look at it if they have time.)

## Build requirements

Unfortunately, this library comes with some non-standard build requirements, due to the fact that `tsf-sys` uses both
[cc](https://crates.io/crates/cc) and [bindgen](https://crates.io/crates/bindgen). This means that you will need to do
some extra setup, even just if this is a dependency (or a dependency of a dependency...).


See the [README for tsf-sys](https://crates.io/crates/tsf-sys) for more information.