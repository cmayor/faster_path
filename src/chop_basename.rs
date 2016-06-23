use std::path::MAIN_SEPARATOR;
use libc::{c_char,c_void,size_t};
use std::ffi::{CStr,CString};
use std::{str,mem,ptr};

#[no_mangle]
pub extern fn chop_basename(string: *const c_char, pointr: *const c_void) -> size_t {
  let c_str = unsafe {
    if string.is_null() { return 0 as size_t }
    CStr::from_ptr(string)
  };
  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
  if r_str.is_empty() { return 0 as size_t }

  let mut offset = 0;
  let mut trailing_slashes = r_str.chars().rev();
  loop {
    match trailing_slashes.next() {
      Some(MAIN_SEPARATOR) => { offset = offset + 1 },
      _                    => { break               },
    }
  }
  
  let r_str = &r_str[0..r_str.len()-offset];
  let base = r_str.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");

  let vectr = vec![
    CString::new(&r_str[0..r_str.len()-base.len()]).unwrap().into_raw(),
    CString::new(base).unwrap().into_raw()
  ];

  unsafe {
    ptr::copy_nonoverlapping(vectr.as_ptr() as *const c_void, pointr as *mut c_void, 1);
  }
  let len = vectr.len() as size_t;
  mem::forget(vectr);
  len
}
