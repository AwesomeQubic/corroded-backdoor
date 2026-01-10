#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _entry() {

}

#[panic_handler]
pub fn handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}