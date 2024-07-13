#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::param::Param;
use rtsmart_std::println;

#[marco_main_use(appname = "rust_param", desc = "Rust example8 app.")]
fn rust_main(_param: Param) {
    for p in _param {
        println!("{}", p);
    }
}