#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::{println, time};
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_bench", desc = "Rust bench app.")]
fn rust_main(_param: Param) {
    let start = time::get_time();
    for i in 0..10 {
        println!("Rust program: Hello, world! {}", i);
    }
    let end = time::get_time();
    println!("Time: {:?}", end   - start);
}