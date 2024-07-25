use libc::*;
use std::fmt;

#[repr(C)]
pub struct libc_data {
    char_: c_char,
    schar_: c_schar,
    uchar_: c_uchar,
    float_: c_float,
    double_: c_double,
    short_: c_short,
    int_: c_int,
    long_: c_long,
    longlong_: c_longlong,
    ushort_: c_ushort,
    ulong_: c_ulong,
    ulonglong_: c_ulonglong,
}

impl libc_data {
    pub fn new() -> Self {
        Self {
            char_: 0,
            schar_: 0,
            uchar_: 0,
            float_: 0.0,
            double_: 0.0,
            short_: 0,
            int_: 0,
            long_: 0,
            longlong_: 0,
            ushort_: 0,
            ulong_: 0,
            ulonglong_: 0,
        }
    }
}

impl fmt::Display for libc_data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "char_: {}\n", self.char_)?;
        write!(f, "schar_: {}\n", self.schar_)?;
        write!(f, "uchar_: {}\n", self.uchar_)?;
        write!(f, "float_: {}\n", self.float_)?;
        write!(f, "double_: {}\n", self.double_)?;
        write!(f, "short_: {}\n", self.short_)?;
        write!(f, "int_: {}\n", self.int_)?;
        write!(f, "long_: {}\n", self.long_)?;
        write!(f, "longlong_: {}\n", self.longlong_)?;
        write!(f, "ushort_: {}\n", self.ushort_)?;
        write!(f, "ulong_: {}\n", self.ulong_)?;
        write!(f, "ulonglong_: {}\n", self.ulonglong_)
    }
}

#[no_mangle]
pub extern "C" fn print_libc_data(data: libc_data) {
    println!("{}", data);
}
