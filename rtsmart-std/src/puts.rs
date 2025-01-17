use core::cmp::min;

fn up_cast(a: usize, b: usize) -> usize {
    let r = a / b;
    return if a % b == 0 { r } else { r + 1 };
}

pub(crate) fn puts(str: &str, kp: fn(s: *const u8)) {
    let str = str.as_bytes();
    let mut buf = [0 as u8; 129];
    for i in 0..up_cast(str.len(), 128) {
        let end = min(128, str.len() - i * 128);
        for j in 0..end {
            buf[j] = str[(j + i * 128) as usize];
        }
        buf[end] = 0;
        kp(buf.as_ptr())
    }
}

