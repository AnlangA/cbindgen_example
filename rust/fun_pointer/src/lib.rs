#[no_mangle]
pub extern "C" fn call_printer(printer: extern "C" fn(*const i8), message: *const i8) {
    // 调用传入的打印函数
    printer(message);
}

