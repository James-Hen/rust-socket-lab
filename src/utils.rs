use libc::{ c_int };

use std::io::{ Error };

/// expand as a C-style string pointers
macro_rules! cstr {
    ($s: expr) => {
        CString::new($s).unwrap().as_ptr()
    };
}