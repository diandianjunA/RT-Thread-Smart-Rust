use libc::rt_sem_t;
use crate::api::semaphore::{sem_create, sem_delete, sem_release, sem_take, sem_take_forever};

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

pub struct Semaphore {
    pub sem: rt_sem_t,
}

impl Semaphore {
    pub fn new(name: &str) -> Option<Self> {
        sem_create(name)
    }

    pub fn take(&self, tick: isize) -> bool {
        sem_take(&self, tick)
    }

    pub fn take_wait_forever(&self) {
        sem_take_forever(&self)
    }

    pub fn release(&self) {
        sem_release(&self)
    }
    
    pub fn drop(&self) {
        sem_delete(&self)
    }
}
