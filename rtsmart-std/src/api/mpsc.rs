use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;

use libc::{c_char, c_void, rt_int32_t, rt_mailbox_t, rt_mq_t, rt_ubase_t, rt_uint32_t};
use crate::println;

pub type MessageQueue = rt_mq_t;

pub fn mq_create(name: &str, len: u32, message_size: u32) -> MessageQueue {
    let name = CString::new(name).unwrap();
    let name_c = name.as_ptr() as *const c_char;
    let mq = unsafe {
        libc::rt_mq_create(name_c, message_size as libc::rt_size_t, len as libc::rt_size_t, 0)
    };
    mq
}

pub fn mq_send(mq: MessageQueue, data: *const c_void, data_size: u32, tick: i32) {
    unsafe {
        libc::rt_mq_send(mq, data, data_size as libc::rt_size_t)
    };
}

pub fn mq_recv(mq: MessageQueue, data: *mut c_void, data_size: u32, tick: i32) {
    unsafe {
        libc::rt_mq_recv(mq, data, data_size as libc::rt_size_t, tick as rt_int32_t)
    };
}

pub fn mq_delete(mq: MessageQueue) {
    unsafe {
        libc::rt_mq_delete(mq);
    }
}
