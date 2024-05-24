use core::time::Duration;

pub fn sleep(time: Duration) {
    unsafe {
        libc::usleep(time.as_micros() as _);
    }
}
