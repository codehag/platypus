use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::collections::HashMap;

#[no_mangle]
pub fn get_data(argu: *const c_char) -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "receive");
    data.insert("Carol", "intercept");

    let foo = unsafe {
       CStr::to_str(CStr::from_ptr(argu))
    };

    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages with {:?}", p, a, foo.unwrap()))
        .collect::<Vec<_>>();

    CString::new(descriptions.join(", "))
        .unwrap()
        .into_raw()
}

fn main() {
    println!("Hello, world!");
}
