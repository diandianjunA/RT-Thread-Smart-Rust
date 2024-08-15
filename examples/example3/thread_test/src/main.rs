#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use marco_main::marco_main_use;
use rtsmart_std::{println, time};
use rtsmart_std::thread::{delay, Thread};
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_thread_test", desc = "Rust example3 app.")]
fn rust_main(_param: Param) {
    println!("Hello world");
    let run1 = || loop {
        time::sleep(Duration::new(1, 0));
        {
            let mut sum = 0;
            for i in 0..10 {
                sum += i;
            }
            println!("thread1: {}", sum);
        }
    };
    let run2 = || loop {
        time::sleep(Duration::new(1, 0));
        {
            let mut sum = 0;
            for i in 0..10 {
                sum += i;
            }
            println!("thread2: {}", sum);
        }
    };

    let t1 = Thread::new()
        .name("thread 1")
        .stack_size(4096)
        .start(run1.clone());
    let t2 = Thread::new()
        .name("thread 2")
        .stack_size(4096)
        .start(run2.clone());
}
