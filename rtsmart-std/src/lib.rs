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
pub mod logging;
pub mod param;
pub mod semaphore;
pub mod mpsc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RTTError {
    ThreadStartupErr,
    MutexTakeTimeout,
    ThreadCreateErr,
    ThreadDeleteErr,
    MutexCreateErr,
    MutexTakeErr,
    MutexReleaseErr,
    MutexDeleteErr,
    FileOpenErr,
    FileCloseErr,
    FileReadErr,
    FileWriteErr,
    FileSeekErr,
    FileFlushErr,
    FileDeleteErr,
    FileSetLengthErr,
    FileSyncErr,
    OutOfMemory
}

pub type RTResult<T> = Result<T, RTTError>;

fn panic_on_atomic_context(s: &str) {
    use core::intrinsics::unlikely;
    let is_irq_context = || unsafe { 
        libc::rt_interrupt_get_nest() != 0
    };
    if unlikely(is_irq_context()) {
        panic!("In irq context {}", s);
    }
}

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