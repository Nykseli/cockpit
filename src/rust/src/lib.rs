use std::ffi::c_void;
use glib_sys::{g_bytes_new, GBytes};

#[no_mangle]
extern "C" fn get_quack() -> *mut GBytes {
    let quack = "Rusty quack message";
    unsafe {
        return g_bytes_new(quack.as_ptr() as *const c_void, quack.len());
    }
}
