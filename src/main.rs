use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
#[no_mangle]
pub fn get_data(argu: *const c_char) -> *mut c_char {

    let test_string = "this is a very long string that has some charaters in it";
    let test_string2 = "this is a very long string that has some charaters in it";
    let test_string3 = "this is a very long string that has some charaters in it";

    let foo = unsafe {
       CStr::to_str(CStr::from_ptr(argu))
    };


    let result = test_string
        .match_indices(foo.unwrap())
        .map(|(i, v)| format!("{}: {}", v, i))
        .collect::<Vec<_>>();

    CString::new(result.join(", "))
        .unwrap()
        .into_raw()
}

fn main() {
    println!("Hello, world!");
}
