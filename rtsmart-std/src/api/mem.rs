//! For malloc.rs
use libc::c_void;
use libc::rt_free;
use libc::rt_malloc;

// Alloc memory
pub fn mem_alloc(bytes: usize) -> *mut c_void {
    unsafe { rt_malloc(bytes as _) }
}
// Free memory
pub fn mem_free(ptr: *mut c_void) {
    unsafe {
        rt_free(ptr);
    }
}
