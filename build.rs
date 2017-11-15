extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-lib=apr-1");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=crypt");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/apr-1.0")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
