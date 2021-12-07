use libc::*;

use std::io::{ Error };

pub use std::ffi::CString;

/// expand as a C-style string pointers
#[macro_export]
macro_rules! cstr {
    ($s: expr) => {
        CString::new($s).unwrap().as_ptr()
    };
}

pub unsafe fn tcp_send(socket: c_int, msg: &String) -> Option<()> {
    let n = write(
        socket,
        msg.as_bytes().as_ptr() as *const c_void,
        msg.len());
    if n <= 0 {
        println!("last OS error: {:?}", Error::last_os_error());
        None
    }
    else {
        Some(())
    }
}

const MAX_BUF: usize = 1460;

pub unsafe fn tcp_recv(socket: c_int) -> Option<String> {
    let mut buf = [0u8; MAX_BUF];
    let n = read(
        socket,
        buf.as_mut_ptr() as *mut c_void,
        buf.len());
    if n <= 0 {
        println!("last OS error: {:?}", Error::last_os_error());
        None
    }
    else {
        Some(std::str::from_utf8(&buf[..n as usize]).unwrap().to_string())
    }
}