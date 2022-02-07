extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let beagle_dst = cmake::Config::new("beagle-lib").define("BUILD_JNI", "OFF").define("BUILD_OPENCL", "ON").cflag("-fPIC").cxxflag("-fPIC").build();

    println!("cargo:rustc-link-search=native={}", beagle_dst.join("lib").display());
    println!("cargo:rustc-link-lib=hmsbeagle");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}/include/libhmsbeagle-1/", beagle_dst.display()))
        .generate()
        .expect("Unable to generate BEAGLE bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write BEAGLE bindings");
} 
