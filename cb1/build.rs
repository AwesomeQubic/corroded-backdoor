use std::env;

use std::{
    path::PathBuf,
    process::Command,
};

fn main() {
    println!("cargo:rustc-link-arg=-T{}/../link.ld", env::var("CARGO_MANIFEST_DIR").unwrap());
}

