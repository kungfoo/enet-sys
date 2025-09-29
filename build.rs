extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Tell Cargo when to rerun the build script
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=vendor/enet/include/");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
        .header("wrapper.h")
        .derive_debug(false)
        .blocklist_type("ENetPacket")
        .blocklist_type("_ENetPacket")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    // Only write bindings if contents changed
    let new_contents = bindings.to_string();
    let write_needed = match fs::read_to_string(&bindings_path) {
        Ok(existing_contents) => existing_contents != new_contents,
        Err(_) => true,
    };

    if write_needed {
        fs::write(&bindings_path, new_contents).expect("Couldn't write bindings!");
    }

    // Build C library
    let dst = Config::new("vendor/enet").build();
    eprintln!("Built ENet to: {}", dst.display());

    // Link platform-specific libraries
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=winmm");
    }

    println!(
        "cargo:rustc-link-search=native={}/lib/static",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=enet");
}
