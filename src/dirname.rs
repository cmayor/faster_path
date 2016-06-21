use std::path::{MAIN_SEPARATOR};
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[no_mangle]
pub extern fn dirname(string: *const c_char) -> *const c_char {
  let c_str = unsafe {
    assert!(!string.is_null());
    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  let mut r = r_str.chars().rev().peekable();
  let mut offset: usize = 0;
  let mut current_dir = false;
  let mut ms_count: usize = 0;
  loop {
    match r.next() {
      Some(MAIN_SEPARATOR) => {
        offset = offset + 1;
        ms_count = ms_count + 1;
        if r.peek().unwrap_or(&MAIN_SEPARATOR).ne(&MAIN_SEPARATOR) && offset != 1 {
          break
        }
      },
      Some(_) => { offset = offset + 1 },
      None => {
        current_dir = true;
        break
      },
    }
  }

  //if current_dir {
  //  CString::new(".").unwrap().into_raw()
  //} else {
    CString::new(&r_str[0..r_str.len()-offset]).unwrap().into_raw()
  //}
}
