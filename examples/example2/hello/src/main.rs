#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::{debug, error, info, log, println, trace, warn};
use rtsmart_std::logging::Level;
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_hello", desc = "Rust example2 app.")]
fn rust_main(_param: Param) {
    println!("hello world");
    log!(Level::Info, "hello world");
    info!("hello world");
    warn!("hello world");
    error!("hello world");
    trace!("hello world");
    debug!("hello world");
    for p in _param {
        println!("{}", p);
    }
}
