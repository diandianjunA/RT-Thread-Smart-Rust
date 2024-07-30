#![no_std]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;

use marco_main::marco_main_use;
use rtsmart_std::{println, thread, time};
use rtsmart_std::mutex::Mutex;
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_mutex_test", desc = "Rust example4 app.")]
fn rust_main(_param: Param) {
    let counter = Arc::new(Mutex::new(0).unwrap());
    let run = move || loop {
        time::sleep(Duration::new(1, 0));
        {
            let mut c = counter.lock().unwrap();
            *c += 1;
            println!("c :{}", *c);
        }
    };

    let t1 = thread::Thread::new()
        .name("thread 1")
        .stack_size(4096)
        .start(run.clone()).unwrap();
    let t2 = thread::Thread::new()
        .name("thread 2")
        .stack_size(4096)
        .start(run.clone()).unwrap();
}
