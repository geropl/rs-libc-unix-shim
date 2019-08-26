extern crate libc_unix_shim_lib as libc;

use core::ffi::c_void;

use libc::{size_t, c_int};

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer: &mut Vec<u8> = &mut vec!(0; 10);
    unsafe { libc::read(0, buffer.as_ptr() as *mut c_void, buffer.capacity()) };
    // println!("read: {}", std::str::from_utf8(buffer).unwrap());

    let write_buffer: &[&[u8]] = &vec!("hello\n".as_bytes());
    let iovecs: Vec<_> = write_buffer.iter().map(|d| {
        libc::iovec { iov_base: d.as_ptr() as *mut c_void, iov_len: d.len() as size_t }
    }).collect();
    let rc = unsafe { libc::writev(1, iovecs.as_ptr(), iovecs.len() as c_int) };
    if rc < 0 {
        Err(Box::new(io::Error::last_os_error()))
    } else {
        Ok(())
    }
}