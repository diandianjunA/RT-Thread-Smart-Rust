#![allow(dead_code)]
#![allow(missing_docs, nonstandard_style)]

use crate::io::ErrorKind;

pub use self::rand::hashmap_random_keys;
pub use crate::os::rtsmart as platform;
pub use libc::strlen;

#[macro_use]
#[path = "../unix/weak.rs"]
pub mod weak;


// #[path = "../unix/ext/mod.rs"]
// pub mod ext;
#[path = "../unix/fd.rs"]
pub mod fd;
#[path = "../unix/fs.rs"]
pub mod fs;
#[path = "../unix/io.rs"]
pub mod io;
// #[path = "../unix/memchr.rs"]
// pub mod memchr;
#[path = "../unix/net.rs"]
pub mod net;
#[path = "../unix/os.rs"]
pub mod os;
#[path = "../../path/mod.rs"]
pub mod path;
#[path = "../unix/pipe.rs"]
pub mod pipe;
#[path = "../../sync/rwlock/mod.rs"]
pub mod rwlock;

#[path = "../unix/stdio.rs"]
pub mod stdio;
#[path = "../unix/time.rs"]
pub mod time;

// 自有的
pub mod env;
pub mod rand;
pub mod thread_local_dtor;
pub mod process;

// 少量修改的
// #[path = "../unix/thread.rs"]
pub mod thread;
pub mod condvar;

// 不需要进行修改的

#[path = "../unix/alloc.rs"]
pub mod alloc;
#[path = "../unix/args.rs"]
pub mod args;
#[path = "../../cmath.rs"]
pub mod cmath;
#[path = "../../sync/mutex/mod.rs"]
pub mod mutex;
#[path = "../unix/thread_local_key.rs"]
pub mod thread_local_key;

// 被屏蔽的
#[path = "../unix/stack_overflow.rs"]
pub mod stack_overflow;

pub use crate::sys_common::os_str_bytes as os_str;

#[cfg(not(test))]
pub fn init() {}

pub fn abort_internal() -> ! {
    loop{}
}

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno as libc::c_int {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::ECONNABORTED => ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => ErrorKind::AddrInUse,
        libc::ENOENT => ErrorKind::NotFound,
        libc::EINTR => ErrorKind::Interrupted,
        libc::EINVAL => ErrorKind::InvalidInput,
        libc::ETIMEDOUT => ErrorKind::TimedOut,
        libc::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}

#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    if t.is_minus_one() { Err(crate::io::Error::last_os_error()) } else { Ok(t) }
}

pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
    where
        T: IsMinusOne,
        F: FnMut() -> T,
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}

pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
    if error == 0 { Ok(()) } else { Err(crate::io::Error::from_raw_os_error(error)) }
}

#[no_mangle]
pub extern "C" fn abort() {

}
