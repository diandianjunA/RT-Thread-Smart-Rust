use alloc::string::String;
use crate::println;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => ({
        $crate::logging::_log($level, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Error, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Warn, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Info, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Debug, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Trace, format_args!($($arg)*));
    });
}

pub fn _log(level: Level, args: core::fmt::Arguments) {
    use core::fmt::Write;
    use crate::stdout;
    use crate::time;
    let mut s = String::new();
    write!(&mut s, "[{:?}][{:?}]", level, time::get_time()).unwrap();
    write!(&mut s, "{}", args).unwrap();
    match level {
        Level::Error => {
            println!("\x1b[1;31m{}\x1b[0m", s);
        }
        Level::Warn => {
            println!("\x1b[1;33m{}\x1b[0m", s);
        }
        Level::Info => {
            println!("\x1b[1;32m{}\x1b[0m", s);
        }
        Level::Debug => {
            println!("\x1b[1;34m{}\x1b[0m", s);
        }
        Level::Trace => {
            println!("\x1b[1;30m{}\x1b[0m", s);
        }
    }
}
