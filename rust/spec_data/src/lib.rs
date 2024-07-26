#![crate_type = "staticlib"]

use std::os::raw::{c_int, c_void};
use std::marker::{PhantomData, PhantomPinned};
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;

// VaList
pub type VaList = *mut c_void;

// RawFd
pub type RawFd = c_int;

// PhantomData
#[repr(C)]
pub struct MyPhantomData<T> {
    _phantom: PhantomData<T>,
}

// PhantomPinned 
#[repr(C)]
pub struct MyPhantomPinned {
    _pinned: PhantomPinned,
}

//()
#[repr(C)]
pub struct MyUnit {
    _unit: (),
}

//MaybeUninit
#[no_mangle]
pub extern "C" fn use_maybe_uninit() -> *mut c_void {
    let mut value: MaybeUninit<i32> = MaybeUninit::uninit();
    unsafe {
        value.as_mut_ptr().write(42);
        value.assume_init() as *mut c_void
    }
}

//ManuallyDrop
#[no_mangle]
pub extern "C" fn use_manually_drop() -> *mut c_void {
    let value = std::mem::ManuallyDrop::new(String::from("Hello"));
    value.as_ptr() as *mut c_void
}

//Pin
#[no_mangle]
pub extern "C" fn use_pin() -> *mut c_void {
    let value = Pin::new(Box::new(42));
    Box::into_raw(Pin::into_inner(value)) as *mut c_void
}

//VaList
#[no_mangle]
pub extern "C" fn use_va_list(_args: VaList) -> c_int {
    // 这里应该有实际的 va_list 处理逻辑
    0
}

//RawFd
#[no_mangle]
pub extern "C" fn use_raw_fd(fd: RawFd) -> c_int {
    // 这里应该有实际的文件描述符处理逻辑
    fd
}

//PhantomData
#[no_mangle]
pub extern "C" fn use_phantom_data() -> *mut c_void {
    let data = MyPhantomData::<i32> { _phantom: PhantomData };
    Box::into_raw(Box::new(data)) as *mut c_void
}

//PhantomPinned
#[no_mangle]
pub extern "C" fn use_phantom_pinned() -> *mut c_void {
    let pinned = MyPhantomPinned { _pinned: PhantomPinned };
    Box::into_raw(Box::new(pinned)) as *mut c_void
}

//()
#[no_mangle]
pub extern "C" fn use_unit() -> *mut c_void {
    let unit = MyUnit { _unit: () };
    Box::into_raw(Box::new(unit)) as *mut c_void
}