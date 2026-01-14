use std::env;

fn main() {
    println!(
        "cargo:rustc-link-arg=-T{}/../link.ld",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
