use alloc::boxed::Box;
use alloc::string::String;
use core::mem;
use libc::{exit, pthread_t, rt_thread_t};
use crate::{println, RTResult, RTTError};
use crate::RTTError::ThreadStartupErr;

pub struct ThreadBuilder {
    th_name: String,
    th_stack_size: u32,
    th_priority: u8,
    th_ticks: u32,
}

impl ThreadBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.th_name = name.into();
        self
    }

    pub fn stack_size(&mut self, stack_size: u32) -> &mut Self {
        self.th_stack_size = stack_size;
        self
    }

    pub fn priority(&mut self, priority: u8) -> &mut Self {
        self.th_priority = priority;
        self
    }

    pub fn ticks(&mut self, ticks: u32) -> &mut Self {
        self.th_ticks = ticks;
        self
    }

    pub fn start<F>(&self, func: F) -> RTResult<Thread>
        where
            F: FnOnce() -> (),
            F: Send + 'static,
    {
        Thread::spawn(
            self.th_name.clone(),
            self.th_stack_size,
            self.th_priority,
            self.th_ticks,
            func,
        )
    }
}

pub struct Thread(pthread_t);

impl Thread {
    pub fn new() -> ThreadBuilder {
        ThreadBuilder {
            th_name: String::new(),
            th_stack_size: 0,
            th_priority: 0,
            th_ticks: 0,
        }
    }

    pub fn spawn<F>(
        name: String,
        stack_size: u32,
        priority: u8,
        ticks: u32,
        func: F,
    ) -> RTResult<Thread>
        where
            F: FnOnce() -> () + Send + 'static,
    {
        unsafe { Self::spawn_inner(name, stack_size, priority, ticks, Box::new(func)) }
    }

    unsafe fn spawn_inner(
        name: String,
        stack_size: u32,
        priority: u8,
        ticks: u32,
        func: Box<dyn FnOnce() -> () + Send + 'static>,
    ) -> RTResult<Thread> {
        let name = name.as_ptr() as *const i8;
        let func = Box::new(func);
        let param = &*func as *const _ as *mut _;
        extern "C" fn thread_func(func: *mut libc::c_void) -> *mut libc::c_void {
            unsafe {
                let func = Box::from_raw(func as *mut Box<dyn FnOnce()>);
                func();
                Thread::thread_exit()
            }
        }
        let mut thread = pthread_t::default();
        let attr = core::ptr::null();
        let ret = crate::api::thread::thread_create(
            &mut thread,
            attr,
            thread_func,
            param,
        );
        if ret != 0 {
            return Err(ThreadStartupErr);
        }
        mem::forget(func);
        Ok(Thread(thread))
    }

    pub fn join(self) {
        let ptr = Box::new(Box::new(0u64));
        let ptr = Box::into_raw(ptr) as *mut _;
        let ret = unsafe {
            crate::api::thread::thread_join(self.0, ptr)
        };
        if ret != 0 {
            println!("thread join failed: {}", ret);
        }
    }

    pub fn thread_exit() -> ! {
        let a = Box::new(0u64);
        let a = Box::into_raw(a) as *mut _;
        crate::api::thread::thread_exit(a)
    }
}