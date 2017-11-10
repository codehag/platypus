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
        forget(bytes);
        let boxed = Box::new(JsBytes { ptr, len, cap });
        Box::into_raw(boxed)
    }
}

#[no_mangle]
pub fn drop_bytes(ptr: *mut JsBytes) {
    unsafe {
        let boxed: Box<JsBytes> = Box::from_raw(ptr);
        Vec::from_raw_parts(boxed.ptr as *mut u8, boxed.len as usize, boxed.cap as usize);
    }
}

fn find(source: &str, search_str: &str) -> Vec<u8> {
    source
        .match_indices(search_str)
        .map(|(i, _)| i as u8)
        .collect::<Vec<u8>>()
}

#[no_mangle]
pub fn get_data(argu: *const c_char) -> *mut JsBytes {

    let test_string = "this is a very long string that has some charaters in it";

    let foo = unsafe {
       CStr::to_str(CStr::from_ptr(argu))
    };

    JsBytes::new(find(test_string, foo.unwrap()))
}

fn main() {
    println!("Hello, world!");
}
