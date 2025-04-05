use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/leanstore/backend/leanstore");
    println!("cargo:rustc-link-search=/leanstore/shared-headers");
    println!("cargo:rustc-link-search=/leanstore/cmake-build-debug/vendor");
    println!("cargo:rustc-link-search=/leanstore/cmake-build-debug/vendor/tabulate/include");

    let bindings = bindgen::Builder::default()
        .header("leanstore/backend/leanstore/c.h")
        .detect_include_paths(true)
        .clang_arg("-I/leanstore/shared-headers")
        .clang_arg("-I/leanstore/cmake-build-debug/vendor")
        .clang_arg("-I/leanstore/cmake-build-debug/vendor/tabulate/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}