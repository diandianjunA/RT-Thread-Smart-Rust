use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use libc::pthread_mutex_t;

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}


pub struct Mutex<T: Sized> {
    mutex: pthread_mutex_t,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: Sized> {
    mutex: &'a pthread_mutex_t,
    data: &'a UnsafeCell<T>,
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
        unsafe {
            libc::pthread_mutex_unlock(mutex);
        }
    }
}

impl <'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data.get() }
    }
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            mutex: unsafe { core::mem::zeroed() },
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        let ptr = &self.mutex as *const _ as *mut _;
        unsafe {
            libc::pthread_mutex_lock(ptr);
        }
        MutexGuard {
            mutex: &self.mutex,
            data: &self.data,
        }
    }
    
    pub fn unlock(&self) {
        let ptr = &self.mutex as *const _ as *mut _;
        unsafe {
            libc::pthread_mutex_unlock(ptr);
        }
    }
}