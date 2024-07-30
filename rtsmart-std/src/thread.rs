use alloc::boxed::Box;
use alloc::string::String;
use core::mem;

use libc::{c_void, rt_thread_t};

use crate::{RTResult, RTTError};
use crate::api::thread::{thread_create, thread_delete, thread_startup, thread_m_delay, thread_self};
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

pub struct Thread(rt_thread_t);

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
        let func = Box::new(func);
        let param = &*func as *const _ as *mut _;

        extern "C" fn thread_func(param: *mut c_void) {
            unsafe {
                let run = Box::from_raw(param as *mut Box<dyn FnOnce()>);
                run();
            }
        }

        let th_handle = thread_create(
            name.as_ref(),
            thread_func,
            param,
            stack_size,
            priority,
            ticks,
        )
            .ok_or(RTTError::OutOfMemory)?;

        let ret = match Self::_startup(th_handle) {
            Ok(_) => {
                mem::forget(func);
                Ok(Thread(th_handle))
            }
            Err(e) => Err(e),
        };

        return ret;
    }
    
    fn _startup(th: rt_thread_t) -> RTResult<()> {
        let ret = thread_startup(th);
        return if ret == 0 {
            Ok(())
        } else {
            Err(ThreadStartupErr)
        };
    }
    
    pub fn delete(&self) -> RTResult<()> {
        let ret = thread_delete(self.0);
        if ret == 0 {
            Ok(())
        } else {
            Err(ThreadStartupErr)
        }
    }
    
    pub fn self_() -> Option<Thread> {
        thread_self().map(Thread)
    }
    
    pub fn raw(&self) -> rt_thread_t {
        self.0
    }
}

pub fn delay(ms: i32) -> RTResult<()> {
    let ret = thread_m_delay(ms);
    if ret == 0 {
        Ok(())
    } else {
        Err(ThreadStartupErr)
    }
}
