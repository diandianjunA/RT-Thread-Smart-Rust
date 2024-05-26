#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::println;

#[marco_main_use(appname = "rust_hello", desc = "Rust example2 app.")]
fn main() {
    println!("hello world");
}
