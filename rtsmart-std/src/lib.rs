#![no_std]
#![feature(alloc_error_handler)]
#![feature(allow_internal_unstable)]
#![feature(linkage)]
#![feature(core_intrinsics)]
#![allow(dead_code)]

#[global_allocator]
static GLOBAL: malloc::RttAlloc = malloc::RttAlloc;

mod api;

pub extern crate alloc;
pub mod out;
pub mod puts;
pub mod prelude;
pub mod malloc;

// TODO: review this enum
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

fn panic_on_atomic_context(s: &str) {
    use crate::api::interrupt::is_irq_context;
    use core::intrinsics::unlikely;
    if unlikely(is_irq_context()) {
        panic!("In irq context {}", s);
    }
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