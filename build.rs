use std::env;
use std::path::PathBuf;

fn main() {
    // TODO(DB): Get rid of static linking and create a build pipeline
    println!("cargo:rustc-link-search=leanstore/backend/leanstore");
    println!("cargo:rustc-link-search=leanstore/shared-headers");
    println!("cargo:rustc-link-search=leanstore/cmake-build-debug/backend");
    println!("cargo:rustc-link-search=leanstore/cmake-build-debug/vendor");
    println!("cargo:rustc-link-search=leanstore/cmake-build-debug/vendor/tabulate/include");
    println!("cargo:rustc-link-lib=tbb");
    println!("cargo:rustc-link-lib=aio");
    println!("cargo:rustc-link-lib=gflags");
    println!("cargo:rustc-link-lib=stdc++");

    println!("cargo:rustc-link-lib=static=leanstore");
    let bindings = bindgen::Builder::default()
        .header("leanstore/backend/leanstore/c.h")
        .detect_include_paths(true)
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}