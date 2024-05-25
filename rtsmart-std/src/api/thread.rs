use alloc::ffi::CString;
use core::ptr;
use libc::{c_char, c_void, pthread_t, rt_err_t, rt_thread_create, rt_thread_delete, rt_thread_mdelay, rt_thread_self, rt_thread_startup, rt_thread_t};

// pub fn get_pid() -> i32 {
//     unsafe { libc::getpid() }
// }
// 
// pub fn get_tid() -> pthread_t {
//     unsafe { libc::pthread_self() }
// }
// 
// pub fn thread_create(
//     thread: *mut pthread_t,
//     attr: *const libc::pthread_attr_t,
//     start_routine: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
//     arg: *mut libc::c_void,
// ) -> i32 {
//     unsafe { libc::pthread_create(thread, attr, start_routine, arg) }
// }
// 
// pub fn thread_join(thread: pthread_t, retval: *mut *mut libc::c_void) -> i32 {
//     unsafe { libc::pthread_join(thread, retval) }
// }
// 
// pub fn thread_exit(retval: *mut libc::c_void) -> ! {
//     unsafe { libc::pthread_exit(retval) }
// }

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
            priority,
            tick,
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

