use alloc::ffi::CString;
use libc::{c_char, RT_IPC_FLAG_FIFO};
use crate::semaphore::Semaphore;

pub fn sem_create(name: &str) -> Option<Semaphore> {
    let name = CString::new(name).unwrap();
    let name_c = name.as_ptr() as *const c_char;
    let sem = unsafe { libc::rt_sem_create(name_c, 0, RT_IPC_FLAG_FIFO) };
    if sem.is_null() {
        None
    } else {
        Some(Semaphore { sem })
    }
}

pub fn sem_take(sem: &Semaphore, tick: isize) -> bool {
    let ret = unsafe { libc::rt_sem_take(sem.sem, tick as _) };
    ret == 0
}

pub fn sem_take_forever(sem: &Semaphore) {
    unsafe {
        libc::rt_sem_take(sem.sem, libc::RT_WAITING_FOREVER);
    }
}

pub fn sem_release(sem: &Semaphore) {
    unsafe {
        libc::rt_sem_release(sem.sem);
    }
}

pub fn sem_delete(sem: &Semaphore) {
    unsafe {
        libc::rt_sem_delete(sem.sem);
    }
}