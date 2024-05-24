#![no_std]
#![no_main]


extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;
use marco_main::marco_main_use;
use rtsmart_std::{println, thread, time};
use rtsmart_std::api::thread::{get_pid, get_tid};
use rtsmart_std::mutex::Mutex;

#[marco_main_use(appname = "rust_mutex_test", desc = "Rust example5 app.")]
fn main(_param: Param) {
    let counter = Arc::new(Mutex::new(0));
    let run = move || {
        time::sleep(Duration::new(2, 0));
        {
            let mut c = counter.lock();
            *c += 1;
            println!("c :{}, pid: {}", *c, get_tid());
            counter.unlock();
        }
    };


    let t1 = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(run.clone());
    time::sleep(Duration::new(1, 0));
    let t2 = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(run.clone());
    t1.unwrap().join();
    t2.unwrap().join();
}
