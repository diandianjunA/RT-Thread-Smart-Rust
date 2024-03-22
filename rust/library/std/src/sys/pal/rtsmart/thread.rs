// use crate::ffi::{CString, CStr};
// use crate::io;
// use crate::time::Duration;
// use crate::mem;

// pub struct Thread {
//     id: libc::rt_thread_t,
// }

// pub const DEFAULT_MIN_STACK_SIZE: usize = 16 * 1024;

// unsafe impl Send for Thread {}
// unsafe impl Sync for Thread {}

// impl Thread {
//     pub unsafe fn new(_stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
//         let p = Box::into_raw(box p);

//         let tid = libc::rt_thread_create(
//             CString::new("un").expect("CString::new failed").into_raw(),
//             thread_start, 
//             p as *mut _,
//             DEFAULT_MIN_STACK_SIZE as _,
//             25,
        //     20
        // );

        // return if tid == 0 as _{
        //     drop(Box::from_raw(p));
        //     Err(io::Error::from_raw_os_error(-1))
        // } else {
        //     let ret = libc::rt_thread_startup(tid);
        //     if ret != 0 {
        //         drop(Box::from_raw(p));
        //         Err(io::Error::from_raw_os_error(ret))
        //     } else {
        //         Ok(Thread { id: tid })
    //         }            
    //     };

    //     extern "C" fn thread_start(main: *mut libc::c_void) {
    //         unsafe {
    //             Box::from_raw(main as *mut Box<dyn FnOnce()>)();
    //         }
    //     }
    // }

    // pub fn yield_now() {
    //     let ret = unsafe { libc::rt_thread_yield() };
    //     debug_assert_eq!(ret, 0);
    // }

    // pub fn set_name(_name: &CStr) {}

    // pub fn sleep(dur: Duration) {
    //     let ms = dur.as_millis();
    //     unsafe {
    //         libc::rt_thread_mdelay(ms as _);
    //     }
    // }

    // pub fn join(self) {
    // }

    // pub fn id(&self) -> libc::pthread_t {
    //     self.id as _
    // }

    // pub fn into_id(self) -> libc::pthread_t {
    //     let id = self.id;
//         mem::forget(self);
//         id as _
//     }
// }

// impl Drop for Thread {
//     fn drop(&mut self) {
//         // let ret = unsafe { libc::rt_thread_delete(self.id) };
//         // debug_assert_eq!(ret, 0);
//     }
// }

// pub mod guard {
//     use crate::ops::Range;
//     pub type Guard = Range<usize>;
//     pub unsafe fn current() -> Option<Guard> {
//         None
//     }
//     pub unsafe fn init() -> Option<Guard> {
//         None
//     }
// }





use crate::ffi::CStr;
use crate::io;
use crate::mem;
use crate::ptr;
use crate::time::Duration;


pub const DEFAULT_MIN_STACK_SIZE: usize = 256 * 1024;

pub struct Thread {
    id: libc::pthread_t,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let p = Box::into_raw(box p);
        let mut native: libc::pthread_t = mem::zeroed();

        let ret = libc::pthread_create(&mut native, 0 as _, thread_start, p as *mut _);

        return if ret != 0 {
            // The thread failed to start and as a result p was not consumed. Therefore, it is
            // safe to reconstruct the box so that it gets deallocated.
            drop(Box::from_raw(p));
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(Thread { id: native })
        };

        extern "C" fn thread_start(main: *mut libc::c_void) -> *mut libc::c_void {
            unsafe {
                Box::from_raw(main as *mut Box<dyn FnOnce()>)();
            }
            ptr::null_mut()
        }
    }

    pub fn yield_now() {
        // let ret = unsafe { libc::sched_yield() };
        // debug_assert_eq!(ret, 0);
    }

    pub fn set_name(_name: &CStr) {
        // Newlib, Haiku, Emscripten, and VxWorks have no way to set a thread name.
    }

    pub fn sleep(dur: Duration) {
        let ms = dur.as_millis();
        unsafe {
            libc::rt_thread_mdelay(ms as _);
        }
    }
    

    pub fn join(self) {
        // unsafe {
        //     let ret = libc::pthread_join(self.id, ptr::null_mut());
        //     mem::forget(self);
        //     assert!(ret == 0, "failed to join thread: {}", io::Error::from_raw_os_error(ret));
        // }
    }

    pub fn id(&self) -> libc::pthread_t {
        self.id
    }

    pub fn into_id(self) -> libc::pthread_t {
        let id = self.id;
        mem::forget(self);
        id
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        // let ret = unsafe { libc::pthread_detach(self.id) };
        // debug_assert_eq!(ret, 0);
    }
}

pub mod guard {
    use crate::ops::Range;
    pub type Guard = Range<usize>;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}