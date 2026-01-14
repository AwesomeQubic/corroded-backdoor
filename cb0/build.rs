use std::env;

use std::{
    path::PathBuf,
    process::Command,
};

fn main() {
    //println!("cargo:rustc-link-arg=-T{}/../link.ld", env::var("CARGO_MANIFEST_DIR").unwrap());

    let status = Command::new("cargo")
        .args([
            "build",
            "-vv",
            "-Z", "build-std=core,alloc",
            "--target", "x86_64-unknown-linux-none",
            "--release",
            "--manifest-path", "../cb1/Cargo.toml",
        ])
        .env("CARGO_TARGET_DIR", format!("{}/../target-cb1", env::var("CARGO_MANIFEST_DIR").unwrap()))
        .status()
        .expect("failed to invoke cargo");

    assert!(status.success());

    let status = Command::new("cargo")
        .args([
            "build",
            "-vv",
            "-Z", "build-std=core,alloc",
            "--target", "x86_64-unknown-linux-none",
            "--release",
            "--manifest-path", "../cb1s/Cargo.toml",
        ])
        .env("CARGO_TARGET_DIR", format!("{}/../target-cb1s", env::var("CARGO_MANIFEST_DIR").unwrap()))
        .status()
        .expect("failed to invoke cargo");

    assert!(status.success());

    // Copy the produced binary into OUT_DIR
    let primary_built_bin = PathBuf::from("../target-cb1/x86_64-unknown-linux-none/release/cb1");
    let secondary_built_bin = PathBuf::from("../target-cb1s/x86_64-unknown-linux-none/release/cb1s");

    println!("cargo:rustc-link-arg=-T{}/../link.ld", env::var("CARGO_MANIFEST_DIR").unwrap());
    // Make Cargo rerun build.rs if the tool changes
    println!("cargo:rerun-if-changed=../cb1/src");
    println!("cargo:rerun-if-changed=../cb1/Cargo.toml");
    println!("cargo:rerun-if-changed=../link.ld");

    // Export path to Rust code
    println!("cargo:rustc-env=CB1_BIN={}", primary_built_bin.canonicalize().unwrap().display());
    println!("cargo:rustc-env=CB1S_BIN={}", secondary_built_bin.canonicalize().unwrap().display());
}

