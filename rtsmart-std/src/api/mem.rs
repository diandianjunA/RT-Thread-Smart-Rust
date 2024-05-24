//! For malloc.rs
use libc::{c_void, free, malloc};

// Alloc memory
pub fn mem_alloc(bytes: usize) -> *mut c_void {
    unsafe { malloc(bytes as _) }
}
// Free memory
pub fn mem_free(ptr: *mut c_void) {
    unsafe {
        free(ptr);
    }
}
