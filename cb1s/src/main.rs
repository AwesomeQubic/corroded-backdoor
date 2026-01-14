#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo, ptr::{null, null_mut}};

#[unsafe(no_mangle)]
pub extern "C" fn _entry() {
    loop {
        int3();
    }
}

#[inline]
//Annoy GDB users
pub fn int3() {
    unsafe {
        asm!(
            "int3"
        );
    }
}

#[panic_handler]
pub fn handler(_: &PanicInfo<'_>) -> ! {
    loop {
        int3();
    }
}
