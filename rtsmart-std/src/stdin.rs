use alloc::string::String;
use core::fmt::Error;
use crate::mutex::Mutex;
use crate::RTTError;

struct Stdin;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, RTTError>;
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, RTTError> {
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
    stdin: Stdin
}

impl InputStream {
    pub fn new() -> InputStream {
        InputStream {
            stdin: Stdin
        }
    }
    
    pub fn read_line(&mut self) -> Result<String, RTTError> {
        let mut buf = [0u8; 1024];
        let mut s = String::new();
        loop {
            let n = self.stdin.read(&mut buf)?;
            if n == 0 {
                break;
            }
            s.push_str(&String::from_utf8_lossy(&buf[..n]));
            if n < buf.len() {
                break;
            } else {
                buf = [0u8; 1024];
            }
        }
        Ok(s)
    }
}

