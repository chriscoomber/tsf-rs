extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = out_path.join("library");

    // Compile tsf_src.h into a library at ${OUT_DIR}/library
    cc::Build::new()
        .include("tsf_src")
        .file("tsf_src/tsf.c")
        // .define("TSF_IMPLEMENTATION", None)
        .out_dir(&lib_dir)
        .compile("libtsf.a");

    // Link the compiled library
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=tsf", );

    // Generate the bindings
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        // Write to our out dir
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}