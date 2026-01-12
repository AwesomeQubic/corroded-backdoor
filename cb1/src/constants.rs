use core::ffi::c_int;

pub const AF_INET: c_int = 2;
pub const SOCK_STREAM: c_int = 1;

pub type sa_family_t = u16;
pub type in_port_t = u16;

pub type in_addr_t = u32;

#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}