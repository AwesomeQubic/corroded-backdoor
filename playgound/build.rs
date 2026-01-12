use std::{
    path::PathBuf,
    process::Command,
};

use std::env;

fn main() {
    let status = Command::new("cargo")
        .args([
            "build",
            "-vv",
            "-Z", "build-std=core,alloc",
            "--target", "x86_64-unknown-linux-none",
            "--release",
            "--manifest-path", "../cb0/Cargo.toml",
        ])
        .env("CARGO_TARGET_DIR", format!("{}/../target-cb0", env::var("CARGO_MANIFEST_DIR").unwrap()))
        .status()
        .expect("failed to invoke cargo");
    assert!(status.success());

    // Copy the produced binary into OUT_DIR
    let built_bin = PathBuf::from("../target-cb0/x86_64-unknown-linux-none/release/cb0");

    // Make Cargo rerun build.rs if the tool changes
    println!("cargo:rerun-if-changed=../link.ld");

    println!("cargo:rerun-if-changed=../cb0/src");
    println!("cargo:rerun-if-changed=../cb0/Cargo.toml");

    println!("cargo:rerun-if-changed=../cb1");

    // Export path to Rust code
    println!("cargo:rustc-env=CB0_BIN={}", built_bin.canonicalize().unwrap().display());
}

