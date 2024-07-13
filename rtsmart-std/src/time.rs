use core::time::Duration;

pub fn sleep(time: Duration) {
    unsafe {
        libc::usleep(time.as_micros() as _);
    }
}

pub fn get_time() -> Duration {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        libc::gettimeofday(&mut tv, core::ptr::null_mut());
    }
    Duration::new(tv.tv_sec as u64, tv.tv_usec as u32 * 1000)
}
