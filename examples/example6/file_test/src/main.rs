#![no_std]
#![no_main]

extern crate alloc;

use marco_main::marco_main_use;
use rtsmart_std::{fs, println};
use rtsmart_std::param::Param;

#[marco_main_use(appname = "rust_file_test", desc = "Rust example6 app.")]
fn rust_main(_param: Param) {
    let res_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open("test.txt");
    if res_file.is_err() {
        println!("{:?}", res_file.err().unwrap());
    } else {
        let file = res_file.unwrap();
        let buf = "Hello, world!".as_bytes();
        file.write(buf).expect("write error");
        let string = file.read_to_string().unwrap();
        println!("{}", string);
        file.close().expect("close error");
    }
}
