#[no_mangle]
pub extern fn __aeabi_memset(_dest: *mut u32, _size: u32, _value: u32) {
    ::abort()
}
