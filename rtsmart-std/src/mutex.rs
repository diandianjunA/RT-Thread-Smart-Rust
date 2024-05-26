use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use libc::rt_mutex_t;

use crate::api::mutex::{mutex_create, mutex_delete, mutex_release, mutex_take};
use crate::RTTError;

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}


pub struct Mutex<T: Sized> {
    mutex: rt_mutex_t,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: Sized> {
    mutex: &'a rt_mutex_t,
    data: &'a UnsafeCell<T>,
}

impl <'a, T> MutexGuard<'a, T> {
    pub fn new(mutex: &'a rt_mutex_t, data: &'a UnsafeCell<T>) -> Self {
        MutexGuard {
            mutex,
            data,
        }
    }
    
    pub fn release(&self) {
        let mutex = self.mutex as *const _ as *mut _;
        mutex_release(mutex);
    }
}

impl <'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.get() }
    }
}

impl <'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        let mutex = self.mutex as *const _ as *mut _;
        mutex_release(mutex);
    }
}

impl <'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data.get() }
    }
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Result<Self, RTTError> {
        Ok(Mutex {
            mutex: mutex_create("Unnamed").unwrap(),
            data: UnsafeCell::new(t),
        })
    }
    
    pub fn new_with_name(t: T, name: &str) -> Result<Self, RTTError> {
        Ok(Mutex {
            mutex: mutex_create(name).unwrap(),
            data: UnsafeCell::new(t),
        })
    }

    pub fn try_lock(&self) -> Result<MutexGuard<T>, RTTError> {
        let ret = mutex_take(self.mutex, 0);
        if ret != 0 {
            return Err(RTTError::MutexTakeTimeout);
        }
        Ok(MutexGuard {
            mutex: &self.mutex,
            data: &self.data,
        })
    }
    
    pub fn lock(&self) -> Result<MutexGuard<T>, RTTError> {
        let ret = mutex_take(self.mutex, -1);
        if ret != 0 {
            return Err(RTTError::MutexTakeTimeout);
        }
        Ok(MutexGuard {
            mutex: &self.mutex,
            data: &self.data,
        })
    }
    
    pub fn delete(&self) {
        let mutex = self.mutex as *const _ as *mut _;
        mutex_delete(mutex);
    }
}