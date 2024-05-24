#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use core::time::Duration;
use marco_main::marco_main_use;
use rtsmart_std::{println, time};
use rtsmart_std::thread::Thread;

#[marco_main_use(appname = "rust_thread_test", desc = "Rust example5 app.")]
fn main(_param: Param) {
    println!("Hello world");
    let run1 = Box::new(|| {
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        println!("thread1: {}", sum);
    });
    let run2 = Box::new(|| {
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        println!("thread2: {}", sum);
    });

    let t1 = Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(run1.clone());
    if t1.is_ok() {
        println!("thread 1 started")
    } else {
        println!("thread 1 start failed")
    }
    let t2 = Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(run2.clone());
    t1.unwrap().join();
    t2.unwrap().join();
}