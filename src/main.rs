use std::{env, process::Command, sync::atomic::AtomicBool};

#[repr(C, align(4096))]
struct Aligned<T: ?Sized>(T);

#[link_section = ".text"]
static TEST_DATA: Aligned<[u8; include_bytes!(env!("CB0_BIN")).len() ]> = Aligned(*include_bytes!(env!("CB0_BIN")));

fn main() {
    println!("{:?}", TEST_DATA.0);
    println!("{:?}", TEST_DATA.0.as_ptr());

    let ptr: extern "C" fn() = unsafe { std::mem::transmute(TEST_DATA.0.as_ptr()) };
    println!("{ptr:?}");
    (ptr)();
    (ptr)();
}
