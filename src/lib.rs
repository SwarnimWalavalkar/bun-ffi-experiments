use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use std::ffi::CStr;
// use libc::c_char;

#[no_mangle]
pub extern "C" fn hash(/*message: *const c_char*/) -> u64 {
  // let c_str: &CStr = unsafe {CStr::from_ptr(message)};
  // let str_slice = c_str.to_str().unwrap();
  // let message: String = String::from(str_slice);
  let message: String = String::from("Hello World!");
  let mut s = DefaultHasher::new();
  let _ = &message.hash(&mut s);
  let computed_hash: u64 = s.finish();
  computed_hash
}