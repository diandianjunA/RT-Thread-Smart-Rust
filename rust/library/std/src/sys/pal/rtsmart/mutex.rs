use crate::cell::UnsafeCell;
use crate::mem::MaybeUninit;
use crate::ffi::CString;

pub struct Mutex {
    inner: UnsafeCell<libc::rt_mutex>
}

pub type MovableMutex = Box<Mutex>;

#[inline]
pub unsafe fn raw(m: &Mutex) -> *mut libc::rt_mutex {
    m.inner.get()
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

#[allow(dead_code)]
impl Mutex {
    pub const fn new() -> Mutex {
        Mutex { inner: UnsafeCell::new(libc::rt_mutex{mtype: 0, data: 0 as _})}
    }
    #[inline]
    pub unsafe fn init(&mut self) {
        let r = libc::rt_mutex_init(
            self.inner.get(),
            CString::new("mutex").expect("CString::new failed").into_raw(),
            0
        );
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn lock(&self) {
        let r = libc::rt_mutex_take(self.inner.get(), -1);
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn unlock(&self) {
        let r = libc::rt_mutex_release(self.inner.get());
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::rt_mutex_take(self.inner.get(), 0) == 0
    }
    #[inline]
    pub unsafe fn destroy(&self) {
        let r = libc::rt_mutex_delete(self.inner.get());
        debug_assert_eq!(r, 0);
    }
}


pub struct ReentrantMutex {
    inner: UnsafeCell<libc::rt_mutex>,
}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    pub const unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { inner: UnsafeCell::new(libc::rt_mutex{mtype: 0, data: 0 as _})}
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        let r = libc::rt_mutex_init(
            self.inner.get(),
            CString::new("mutex").expect("CString::new failed").into_raw(),
            0
        );
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn lock(&self) {
        let r = libc::rt_mutex_take(self.inner.get(), -1);
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn unlock(&self) {
        let r = libc::rt_mutex_release(self.inner.get());
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::rt_mutex_take(self.inner.get(), 0) == 0
    }
    #[inline]
    pub unsafe fn destroy(&self) {
        let r = libc::rt_mutex_delete(self.inner.get());
        debug_assert_eq!(r, 0);
    }
}

struct PthreadMutexAttr<'a>(&'a mut MaybeUninit<libc::pthread_mutexattr_t>);

impl Drop for PthreadMutexAttr<'_> {
    fn drop(&mut self) {
        unsafe {
            let result = libc::pthread_mutexattr_destroy(self.0.as_mut_ptr());
            debug_assert_eq!(result, 0);
        }
    }
}
