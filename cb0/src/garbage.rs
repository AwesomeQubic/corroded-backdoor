use core::hint::black_box;

static mut GLOBAL: u64 = 0;
#[inline]
pub fn _dummy() {
    {
        let _is_dummy_145 = true;
        let mut _dummy_counter = 4i32;
        let _dummy_upper_bound = 100;
        let _dummy_increment = 3i32;
        loop {
            if black_box(_dummy_counter > _dummy_upper_bound) {
                unsafe { GLOBAL += 1 };
                break;
            }
            unsafe {
                core::ptr::write_volatile(
                    &mut _dummy_counter,
                    _dummy_counter + _dummy_increment,
                );
            }
        }
    };
    let a = 1;
    let b = 2;
    let c = a + b;
    unsafe { GLOBAL += c };
}
