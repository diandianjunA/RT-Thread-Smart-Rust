use alloc::string::{String, ToString};
use libc::{c_char, c_void, SEEK_SET};
use crate::println;

pub struct File {
    pub fd: i32,
}

pub struct OpenOptions {
    pub path: String,
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub append: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            path: String::new(),
            read: false,
            write: false,
            create: false,
            append: false,
        }
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    pub fn open(&mut self, path: &str) -> Result<File, String> {
        self.path = path.to_string();
        let fd = unsafe {
            crate::fs::open(
                self.path.as_ptr(),
                self.read,
                self.write,
                self.create,
                self.append,
            )
        };
        if fd < 0 {
            Err("open file failed".to_string())
        } else {
            Ok(File { fd })
        }
    }
}

pub fn open(path: *const u8, read: bool, write: bool, create: bool, append: bool) -> i32 {
    let flags = if read && write {
        libc::O_RDWR
    } else if write {
        libc::O_WRONLY
    } else {
        libc::O_RDONLY
    };

    let flags = if create {
        flags | libc::O_CREAT
    } else {
        flags
    };

    let flags = if append {
        flags | libc::O_APPEND
    } else {
        flags
    };
    unsafe { libc::open(path as *const c_char, flags) }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

impl File {
    pub fn read_to_string(&self) -> Result<String, String> {
        let mut buf = [0; 128];
        let mut string = String::new();
        self.seek(0)?;
        loop {
            let n = self.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let substr = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
            string.push_str(&substr);
        }
        Ok(string)
    }
    
    pub fn write_all(&self, buf: &str) -> Result<(), String> {
        self.write(buf.as_bytes())?;
        Ok(())
    }
    
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, String> {
        let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if n < 0 {
            Err("read file failed".to_string())
        } else {
            Ok(n as usize)
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, String> {
        let n = unsafe { libc::write(self.fd, buf.as_ptr() as *const c_void, buf.len()) };
        if n < 0 {
            Err("write file failed".to_string())
        } else {
            Ok(n as usize)
        }
    }

    pub fn seek(&self, offset: i64) -> Result<i64, String> {
        let n = unsafe { libc::lseek(self.fd, offset, SEEK_SET) };
        if n < 0 {
            Err("seek file failed".to_string())
        } else {
            Ok(n)
        }
    }
    
    pub fn flush(&self) -> Result<(), String> {
        let n = unsafe { libc::fsync(self.fd) };
        if n < 0 {
            Err("flush file failed".to_string())
        } else {
            Ok(())
        }
    }
    
    pub fn set_len(&self, len: i64) -> Result<(), String> {
        let n = unsafe { libc::ftruncate(self.fd, len) };
        if n < 0 {
            Err("set_len file failed".to_string())
        } else {
            Ok(())
        }
    }
    
    pub fn sync_all(&self) -> Result<(), String> {
        let n = unsafe { libc::fsync(self.fd) };
        if n < 0 {
            Err("sync_all file failed".to_string())
        } else {
            Ok(())
        }
    }
    
    pub fn close(&self) -> Result<(), String> {
        let n = unsafe { libc::close(self.fd) };
        if n < 0 {
            Err("close file failed".to_string())
        } else {
            Ok(())
        }
    }
}
