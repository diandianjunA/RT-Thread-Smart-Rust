#![no_std]
#![feature(alloc_error_handler)]
#![feature(allow_internal_unstable)]
#![feature(linkage)]
#![feature(core_intrinsics)]
#![allow(dead_code)]

#[global_allocator]
static GLOBAL: malloc::RttAlloc = malloc::RttAlloc;

pub mod api;

pub extern crate alloc;
pub mod stdout;
pub mod puts;
pub mod prelude;
pub mod malloc;
pub mod time;
pub mod thread;
pub mod mutex;
pub mod stdin;
pub mod fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RTTError {
    ThreadStartupErr,
    MutexTakeTimeout,
    SemaphoreTakeTimeout,
    QueueSendTimeout,
    QueueReceiveTimeout,
    OutOfMemory,

    DeviceNotFound,
    DeviceOpenFailed,
    DeviceCloseFailed,
    DeviceReadFailed,
    DeviceWriteFailed,
    DeviceTransFailed,
    DeviceConfigFailed,
    DeviceSetRxCallBackFailed,
    DeviceSetTxCallBackFailed,

    FuncUnDefine,
}

pub type RTResult<T> = Result<T, RTTError>;

#[panic_handler]
#[inline(never)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{:}", info);
    __rust_panic()
}

#[linkage = "weak"]
#[no_mangle]
fn __rust_panic() -> ! {
    loop {}
}