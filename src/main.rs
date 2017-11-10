use std::os::raw::c_char;
use std::ffi::CStr;
use std::mem::forget;

#[repr(C)]
#[derive(Debug)]
pub struct JsBytes {
    ptr: u32,
    len: u32,
    cap: u32,
}

impl JsBytes {
    pub fn new(mut bytes: Vec<u8>) -> *mut JsBytes {
        let ptr = bytes.as_mut_ptr() as u32;
        let len = bytes.len() as u32;
        let cap = bytes.capacity() as u32;
        forget(bytes); // do not drop until we tell you to!
        let boxed = Box::new(JsBytes { ptr, len, cap });
        Box::into_raw(boxed)
    }
}

#[no_mangle]
pub fn drop_bytes(ptr: *mut JsBytes) {
    unsafe {
        Box::from_raw(ptr);
    }
}

fn find(source: &str, search_str: &str) -> Vec<u8> {
    source
        .match_indices(search_str)
        .map(|(i, _)| i as u8)
        .collect::<Vec<u8>>()
}

#[no_mangle]
pub fn search_text(source_text: *const c_char, search_term: *const c_char) -> *mut JsBytes {
    let source_text = unsafe {
       CStr::to_str(CStr::from_ptr(source_text)).unwrap()
    };

    let search_term = unsafe {
       CStr::to_str(CStr::from_ptr(search_term)).unwrap()
    };

    JsBytes::new(find(source_text, search_term))
}

fn main() {
}
