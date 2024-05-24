use alloc::string::String;
use core::fmt::Error;
use crate::mutex::Mutex;

struct Stdin;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut i = 0;
        unsafe {
            while i < buf.len() {
                let c = libc::getchar();
                if c == 0 || c == '\n' as i32 {
                    break;
                }
                buf[i] = c as u8;
                i += 1;
            }
        }
        Ok(i)
    }
}

pub struct InputStream {
    stdin: Mutex<Stdin>
}

impl InputStream {
    pub fn new() -> InputStream {
        InputStream {
            stdin: Mutex::new(Stdin)
        }
    }
    
    pub fn read_line(&mut self) -> Result<String, Error> {
        let mut _lock = self.stdin.lock();
        let mut buf = [0; 128];
        let n = _lock.read(&mut buf)?;
        let string = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
        Ok(string)
    }
}

