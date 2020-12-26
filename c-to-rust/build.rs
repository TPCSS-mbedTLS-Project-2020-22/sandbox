extern crate cc;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/test-main.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/test-main.c")
        .compile("test-main");
}

