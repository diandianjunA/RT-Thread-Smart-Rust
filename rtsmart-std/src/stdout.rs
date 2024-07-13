//! Provide the output function of debugging serial port

use libc::printf;
use crate::puts::puts;
use core::fmt::{self, Write};

struct StdOut;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        fn rtt_kputs(s: *const u8) {
            unsafe { printf(s as _); }
        }
        puts(s, rtt_kputs);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    unsafe {
        StdOut.write_fmt(args).unwrap_unchecked();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::stdout::_print(format_args!($($arg)*));
    });
}

#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::stdout::_print(format_args_nl!($($arg)*));
    });
}
