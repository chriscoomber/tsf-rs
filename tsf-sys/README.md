# tsf-sys &emsp; [![Latest Version]][crates.io] 

[Latest Version]: https://img.shields.io/crates/v/tsf-sys.svg
[crates.io]: https://crates.io/crates/tsf-sys

Unsafe rust bindings for [TinySoundFont](https://github.com/schellingb/TinySoundFont/). Don't use this crate directly, 
instead use `tsf` which has more user-friendly bindings.

## Build requirements

Unfortunately, this library comes with some non-standard build requirements, due to the fact that it uses both 
[cc](https://crates.io/crates/cc) and [bindgen](https://crates.io/crates/bindgen). This means that you will need to do 
some extra setup, even just if this is a dependency (or a dependency of a dependency...).

### cc

We use CC to build the C library which is bundled with this crate. This means you need to have a C compiler installed.
In particular, when building against a target triple (with `cargo build --target x-y-z`) you will need a C compiler
from that target's toolchain (i.e. a C compiler which runs on the host, which compiles to the target). You also need an
archiver for the target.

Practically, this means:
- When compiling with no target (or a target equal to the host) (e.g. for testing), you just need a C compiler and archiver installed in 
  some sensible way; it should be picked up automatically. If it's not (e.g. the wrong one is picked up), then set the 
  `CC` and `AR` environment variables with the paths of the respective executables.
- When compiling with a target triple `x-y-z` different to the host, you will need to set environment variables 
  `CC_x-y-z` and `AR_x-y-z` with paths of the compiler and archiver of the target's toolchain respectively. These are 
  usually the same paths as the ones in your `config` or `config.toml` file, which you probably added when installing
  this target via rustup.

(You may need to set some additional parameters to ensure your compiler chooses he correct target, e.g. the `--target`
clang flag. So far I've only tested Android which doesn't require it if you follow the example below.)

For example, when building to Android you can find this compiler and archiver within the NDK.
See [this guide](https://developer.android.com/ndk/guides/other_build_systems). The compiler will be
`$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin/$TARGET$NDK_VERSION-clang` and the AR will be
`$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin/$TARGET-ar`

These requirements are the same for any project using `cc` to compile a C library, so if you already have this 
configured then your current configuration is probably fine too.

These requirements could be lifted by prebuilding the library for some targets and checking them into git. Pull requests
are welcome!

### Bindgen

We use Bindgen to generate the rust bindings to the C library's header files. This requires an
[installation of clang](https://rust-lang.github.io/rust-bindgen/requirements.html).

These requirements could be liften by prebuilding the bindings for some targets and checking them into git. Pull 
requests are welcome!

### Usage example

Suppose the crate `mylib` depends on `tsf` (which depends on `tsf-sys`), and my host machine is windows and I want to 
compile `mylib` against the target `armv7-linux-androideabi`. Suppose `mylib` produces a dynamic C lib. 

1. Make sure to download the target with `rustup target add armv7-linux-androideabi`. 
   
2. Then download the toolchain of that target. In this case, it's the NDK that you 
   wish to compile against, which for me is NDK 21.
   
3. Add the following to a cargo config file (`config.toml` or `config`):

```
[target.armv7-linux-androideabi]
linker = "C:\\path\\to\\ndk\\21.4.7075529\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\armv7a-linux-androideabi21-clang.cmd"
ar = "C:\\path\\to\\ndk\\21.4.7075529\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\arm-linux-androideabi-ar"
# Note that for this target specifically, the `ar` doesn't include the v7a but of the target. Just a quirk of the NDK.
```

4. Set the following environment variables (you can either paste this into your command prompt, or set them more 
   permanently):

```
set CC_armv7-linux-androideabi=C:\\path\\to\\ndk\\21.4.7075529\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\armv7a-linux-androideabi21-clang.cmd
set AR_armv7-linux-androideabi=C:\\path\\to\\ndk\\21.4.7075529\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\arm-linux-androideabi-ar 
```

5. Follow the bindgen requirements above, including setting the `LIBCLANG_PATH` environment variable.

6. Finally, use cargo to build `mylib`.

```
rem Comments in windows command line are given by rem
rem Print out our environment variables to check they were set right. You can obviously omit this
echo %CC_armv7-linux-androideabi%
echo %AR_armv7-linux-androideabi%
echo %LIBCLANG_PATH%
cargo build --target armv7-linux-androideabi --release
```

For non-Windows host machines all the steps are the same, just slightly different commands. E.g. to set environment
variables you can put `VAR=VALUE ` before the `cargo build` command, which is nice. 

## Contributing

This crate was written by following 
[this rust-bindgen tutorial](https://rust-lang.github.io/rust-bindgen/library-usage.html). If you can follow that 
tutorial, then you understand how this crate works.

## TinySoundFont is bundled

TinySoundFont is distributed as a header file. This crate includes a static library compiled from that header file,
which makes it incredibly convenient to use - you don't need to do anything special.

An un-bundled version of this crate is currently not available. Pull equensts are welcome.
