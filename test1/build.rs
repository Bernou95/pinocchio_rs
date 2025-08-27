use std::env;
use std::path::{Path, PathBuf};
use bindgen;

fn main() {
    // Tell Cargo to rerun the build script if any C++ files change
    println!("cargo:rerun-if-changed=cpp_src");

    // Path to the pre-built shared library (libpinocchio_example.so)
    let lib_path = Path::new("cpp_src/build");

    // Ensure the shared library is at the correct location
    if !lib_path.exists() {
        panic!("Shared library libpinocchio_example.so not found at {:?}", lib_path);
    }

    // Tell Cargo where to look for the shared library
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Link the shared library (libpinocchio_example.so)
    println!("cargo:rustc-link-lib=dylib=pinocchio_example");
    // 3) Embed RPATH so the runtime loader finds the .so near the Rust binary
    // target/debug/<binary>  --> ORIGIN
    // .so is in ../../cpp_wrapper/build  relative to that
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../cpp_src/build"
    );

    // Generate Rust bindings for the C++ functions (if needed)
    /*let out_dir = env::var("OUT_DIR").unwrap();
    let bindings = bindgen::Builder::default()
        .header("cpp_src/pinocchio_example.cpp")  // Use the correct header file location
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to a Rust file in the OUT_DIR
    let out_path = PathBuf::from(out_dir).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");*/

    // Re-run the build script if cpp_src/pinocchio_example.cpp changes
    println!("cargo:rerun-if-changed=cpp_src/pinocchio_example.cpp");
}

