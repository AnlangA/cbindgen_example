use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::slice;

///using a field-less enum in FFI to model a C enum is often wrong. like:
///#[repr(C)]
///pub enum EnumData {
///    Data1,
///    Data2,
///    Data3,
///}
///if you want learn more, just go to the curl: https://doc.rust-lang.org/nightly/reference/type-layout.html#reprc-field-less-enums

#[repr(C)]
pub enum EnumData {
    Data1(*mut c_char),
    Data2(u16),
    Data3(*mut i8, size_t),
}

#[no_mangle]
pub extern "C" fn new_enum_data(
    variant: u32,
    arg1: *const c_char,
    arg2: u16,
    arg3: *const i8,
    arg3_len: libc::size_t,
) -> *mut EnumData {
    let enum_data = match variant {
        0 => {
            if arg1.is_null() {
                return std::ptr::null_mut();
            }
            let c_str = unsafe { CStr::from_ptr(arg1) };
            let owned_string = match c_str.to_str() {
                Ok(s) => CString::new(s).unwrap(),
                Err(_) => return std::ptr::null_mut(),
            };
            EnumData::Data1(owned_string.into_raw())
        }
        1 => EnumData::Data2(arg2),
        2 => {
            if arg3.is_null() {
                return std::ptr::null_mut();
            }
            let vec = unsafe { slice::from_raw_parts(arg3, arg3_len).to_vec() };
            let boxed_slice = vec.into_boxed_slice();
            let ptr = Box::into_raw(boxed_slice) as *mut i8;
            EnumData::Data3(ptr, arg3_len)
        }
        _ => return std::ptr::null_mut(),
    };

    Box::into_raw(Box::new(enum_data))
}

#[no_mangle]
pub extern "C" fn print_enum_data(data: *const EnumData) {
    if data.is_null() {
        println!("Null pointer passed to print_enum_data");
        return;
    }

    // Safety: We've checked that data is not null, and we trust that the caller
    // has passed a valid EnumData pointer. We're only reading from this memory.
    unsafe {
        match *data {
            EnumData::Data1(s) => {
                if !s.is_null() {
                    let c_str = CStr::from_ptr(s);
                    if let Ok(str_slice) = c_str.to_str() {
                        println!("Data1: {}", str_slice);
                    } else {
                        println!("Data1: <invalid UTF-8>");
                    }
                } else {
                    println!("Data1: <null>");
                }
            }
            EnumData::Data2(n) => println!("Data2: {}", n),
            EnumData::Data3(ptr, len) => {
                if !ptr.is_null() {
                    let slice = slice::from_raw_parts(ptr, len);
                    println!("Data3: {:?}", slice);
                } else {
                    println!("Data3: <null>");
                }
            }
        }
    }
}
