#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use core::time::Duration;
use marco_main::marco_main_use;
use rtsmart_std::{mpsc, println, time};
use rtsmart_std::param::Param;
use rtsmart_std::thread::Thread;

#[marco_main_use(appname = "rust_channel_test", desc = "Rust example10 app.")]
fn rust_main(_param: Param) {
    let (tx, rx) = mpsc::channel("rust_channel_test", 2);
    let run1 = move || {
        loop {
            tx.send(String::from("msg"));
            time::sleep(Duration::new(1, 0));
        }
    };
    let run2 = move || {
        loop {
            time::sleep(Duration::new(1, 0));
            println!("waiting!");
            let a = rx.recv().unwrap();
            println!("recv {}", a);
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

