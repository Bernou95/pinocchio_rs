use std::process::Command;

fn main() {
    // 1) Build the C++ wrapper through CMake
    let status = Command::new("cmake")
        .args(&["-S", "cpp_wrapper", "-B", "cpp_wrapper/build"])
        .status()
        .expect("Failed to run CMake");
    assert!(status.success(), "CMake configuration failed!");

    let status = Command::new("cmake")
        .args(&["--build", "cpp_wrapper/build"])
        .status()
        .expect("Failed to build C++ wrapper");
    assert!(status.success(), "Building C++ wrapper failed!");

    // 2) Tell rustc to link against the wrapper
    println!("cargo:rustc-link-search=native=cpp_wrapper/build");
    println!("cargo:rustc-link-lib=dylib=pinocchio_wrapper");

    // 3) Embed RPATH so the runtime loader finds the .so near the Rust binary
    // target/debug/<binary>  --> ORIGIN
    // .so is in ../../cpp_wrapper/build  relative to that
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../cpp_wrapper/build"
    );

    // 4) Only re-trigger wrapper build on changes
    println!("cargo:rerun-if-changed=cpp_wrapper/pinocchio_wrapper.cpp");
    println!("cargo:rerun-if-changed=cpp_wrapper/pinocchio_wrapper.h");
    println!("cargo:rerun-if-changed=cpp_wrapper/CMakeLists.txt");
}

