extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the networkit shared library.
    println!("cargo:rustc-link-lib=networkit");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .whitelist_type("NetworKit::Graph")
        .clang_arg("-I/usr/local/include/networkit/")
        .clang_arg("-I/usr/include/c++/9/")
        .clang_arg("-I/usr/include/c++/9/x86_64-redhat-linux/")
        .clang_arg("-x").clang_arg("c++")
        .clang_arg("-std=c++11")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/bindings.rs");
    // Write the bindings to the src/bindings.rs file.
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
