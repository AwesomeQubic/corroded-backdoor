
#[repr(C, align(16))]
struct Aligned<T: ?Sized>(T);

static mut TEST_DATA: Aligned<[u8; include_bytes!(env!("CB1_BIN")).len() ]> = gen_crypt();//Aligned(*include_bytes!(env!("CB0_BIN")));

pub const fn gen_crypt() -> Aligned<[ u8; include_bytes!(env!("CB1_BIN")).len() ]> {
    let mut initial = *include_bytes!(env!("CB1_BIN"));

    Aligned(initial)
}

pub fn run() {

}