#![no_main]
#![no_std]

use core::{arch::naked_asm, panic::PanicInfo, sync::atomic::AtomicBool};

use linux_syscall::{SYS_write, syscall};

pub mod sys;
pub mod garbage;
pub mod grain;
pub mod payload;

unsafe extern "C" {
    static _backdoor_end: u64;
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn _entry() {
    naked_asm!(
        "lea r10, [rip]",

        "mov rax, QWORD PTR [rip + {latch}]",
        "cmp rax, 1",
        "je 1f",

        //Setup memory
        "mov rax, 10",
        "mov rdi, r10",
        "and rdi, {mask}",
        "mov rsi, {range}",
        "mov rdx, {prot}",
        "syscall",

        "mov rdi, r10",
        "call {start}",

        "1: ret",
        //TODO make it better
        range = const 4096,
        mask = const u64::MAX << 12,
        prot = const sys::PROT_EXECUTE | sys::PROT_WRITE,
        start = sym _start,
        latch = sym LATCH,
    )
}

static LATCH: AtomicBool = AtomicBool::new(false);

pub extern "C" fn _start(load_location: usize) {
    if LATCH.compare_exchange(false, true, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed).is_err() {
        return;
    }
    
    garbage::_dummy();
    let stdout: i32 = 1;
    let hello = "Hello, world!\n\0";
    garbage::_dummy();

    let rc = unsafe {
	    syscall!(SYS_write, stdout, hello.as_ptr(), hello.len())
    };

    garbage::_dummy();
}

#[panic_handler]
pub fn handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}