#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use marco_main::marco_main_use;
use rtsmart_std::{fs, println};

#[marco_main_use(appname = "rust_file_test", desc = "Rust example6 app.")]
fn main(_param: Param) {
    let mut res_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open("test.txt");
    if res_file.is_err() {
        println!("{:?}", res_file.err().unwrap());
    } else {
        let mut file = res_file.unwrap();
        let buf = "Hello, world!".as_bytes();
        file.write(buf).expect("write error");
        let string = file.read_to_string().unwrap();
        println!("{}", string);
        file.close().expect("close error");
    }
}
