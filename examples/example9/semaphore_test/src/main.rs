#![no_std]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;
use marco_main::marco_main_use;
use rtsmart_std::param::Param;
use rtsmart_std::semaphore::Semaphore;
use rtsmart_std::{println, thread, time};

#[marco_main_use(appname = "rust_sem_test", desc = "Rust example9 app.")]
fn rust_main(_param: Param) {
    let send = Arc::new(Semaphore::new("Semaphore").unwrap());
    let recv = send.clone();

    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(move || {
            loop {
                time::sleep(Duration::new(1, 0));
                send.release()
            }
        });
    time::sleep(Duration::new(1, 0));
    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("waiting!");
                recv.take_wait_forever();
                println!("recv a sem!")
            }
        });
}

