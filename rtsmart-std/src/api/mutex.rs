use alloc::ffi::CString;

use libc::{rt_err_t, rt_mutex_create, rt_mutex_delete, rt_mutex_release, rt_mutex_t, rt_mutex_take};

pub fn mutex_create(name: &str) -> Option<rt_mutex_t> {
    let s = CString::new(name).unwrap();
    let s = s.as_ptr() as *const u8;
    let raw;
    unsafe {
        raw = rt_mutex_create(s, 1);
    }
    if raw == core::ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

pub fn mutex_delete(handle: rt_mutex_t) -> rt_err_t {
    unsafe {
        rt_mutex_delete(handle)
    }
}

pub fn mutex_take(handle: rt_mutex_t, tick: isize) -> rt_err_t {
    unsafe { rt_mutex_take(handle, tick as _) }
}

pub fn mutex_release(handle: rt_mutex_t) -> rt_err_t {
    unsafe {
        rt_mutex_release(handle)
    }
}
