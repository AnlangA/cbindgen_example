use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn pointer_hello(c_string: *const c_char) {
    if c_string.is_null() {
        eprintln!("Received a null pointer.");
        return;
    }
    
    let c_str = unsafe { CStr::from_ptr(c_string) };
    
    match c_str.to_str() {
        Ok(rust_str) => {
            println!("Hello, {}!", rust_str);
        }
        Err(e) => {
            eprintln!("Failed to convert C string to Rust string: {}", e);
        }
    }
}
