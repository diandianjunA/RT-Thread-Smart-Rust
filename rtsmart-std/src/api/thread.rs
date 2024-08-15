use alloc::ffi::CString;
use core::ptr;
use libc::{c_char, c_void, pthread_t, rt_err_t, rt_thread_create, rt_thread_delete, rt_thread_mdelay, rt_thread_self, rt_thread_startup, rt_thread_t};

pub type ThreadEntry = extern "C" fn(parameter: *mut c_void);

pub fn thread_create(
    name: &str,
    entry: ThreadEntry,
    param: *mut c_void,
    stack_size: u32,
    priority: u8,
    tick: u32,
) -> Option<rt_thread_t> {
    let name = CString::new(name).unwrap();
    let raw;
    let name_ptr = name.as_ptr() as *const c_char;
    unsafe {
        raw = rt_thread_create(
            name_ptr,
            entry,
            param,
            stack_size,
            25,
            200,
        );
    }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

pub fn thread_startup(th: rt_thread_t) -> rt_err_t {
    unsafe { rt_thread_startup(th) }
}

// Thread have a ms sleep
#[inline]
pub fn thread_m_delay(ms: i32) -> rt_err_t {
    unsafe { rt_thread_mdelay(ms as _) }
}

pub fn thread_delete(th: rt_thread_t) -> rt_err_t {
    unsafe { rt_thread_delete(th).into() }
}

// Get current thread
pub fn thread_self() -> Option<rt_thread_t> {
    let ret;
    unsafe {
        ret = rt_thread_self();
    }
    if ret == ptr::null_mut() {
        None
    } else {
        Some(ret)
    }
}

