#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::println;
use rtsmart_std::stdin::InputStream;
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_read_test", desc = "Rust example5 app.")]
fn rust_main(_param: Param) {
    let mut input = InputStream::new();
    let line = input.read_line().unwrap();
    println!("{}", line);
}
