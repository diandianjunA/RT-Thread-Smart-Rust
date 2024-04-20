#![no_std]
#![no_main]

use marco_main::marco_main_use;
use rtsmart_std::println;

#[marco_main_use(appname = "rust_em_mutex", desc = "Rust example app.")]
fn main(_param: Param) {
    println!("hello world");
}
