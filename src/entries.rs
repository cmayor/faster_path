use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;
use std::fs;
use std::path::Path;
use ruby_array::RubyArray;

#[no_mangle]
pub extern fn entries(string: *const c_char) -> RubyArray {
    let c_str = unsafe {
        assert!(!string.is_null());

        CStr::from_ptr(string)
    };
    let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
    let files = fs::read_dir(Path::new(r_str));

    RubyArray::from_vec(files.iter().map(|&x| CString::new(x).unwrap()).collect::<Vec<c_char>>())
}
