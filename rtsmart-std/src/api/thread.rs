use libc::pthread_t;

pub fn get_pid() -> i32 {
    unsafe { libc::getpid() }
}

pub fn get_tid() -> pthread_t {
    unsafe { libc::pthread_self() }
}

pub fn thread_create(
    thread: *mut pthread_t,
    attr: *const libc::pthread_attr_t,
    start_routine: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    arg: *mut libc::c_void,
) -> i32 {
    unsafe { libc::pthread_create(thread, attr, start_routine, arg) }
}

pub fn thread_join(thread: pthread_t, retval: *mut *mut libc::c_void) -> i32 {
    unsafe { libc::pthread_join(thread, retval) }
}

pub fn thread_exit(retval: *mut libc::c_void) -> ! {
    unsafe { libc::pthread_exit(retval) }
}
