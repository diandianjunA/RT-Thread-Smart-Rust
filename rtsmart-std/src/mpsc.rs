use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem::{MaybeUninit, size_of};

use libc::c_void;

use crate::api::mpsc::{MessageQueue, mq_create, mq_delete, mq_recv, mq_send};

unsafe impl<T> Send for Node<T> {}
unsafe impl<T> Sync for Node<T> {}

#[derive(Clone)]
pub struct Node<T> {
    pub mq: MessageQueue,
    item_type: PhantomData<UnsafeCell<T>>,
}

impl <T>Node<T> {
    pub fn new(mq: MessageQueue) -> Option<Self> {
        Some(Node { 
            mq,
            item_type: PhantomData,
        })
    }
    
    pub fn send(&self, item: T) {
        let inner = MaybeUninit::new(item);
        mq_send(self.mq, inner.as_ptr() as *const c_void, size_of::<T>() as _, libc::RT_WAITING_FOREVER)
    }
    
    pub fn send_timeout(&self, item: T, timeout: i32) {
        let inner = MaybeUninit::new(item);
        mq_send(self.mq, inner.as_ptr() as *const c_void, size_of::<T>() as _, timeout)
    }

    pub fn recv(&self) -> Option<T> {
        let mut item = MaybeUninit::<T>::uninit();
        mq_recv(self.mq, item.as_mut_ptr() as *mut c_void, size_of::<T>() as _, libc::RT_WAITING_FOREVER);
        Some(unsafe { item.assume_init() })
    }

    pub fn recv_timeout(&self, timeout: i32) -> Option<T> {
        let mut item = MaybeUninit::<T>::uninit();
        mq_recv(self.mq, item.as_mut_ptr() as *mut c_void, size_of::<T>() as _, timeout);
        Some(unsafe { item.assume_init() })
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        mq_delete(self.mq);
    }
}


pub fn channel<T>(name: &str, num: u32) -> (Arc<Node<T>>, Arc<Node<T>>) {
    let mb = mq_create(name, num, size_of::<T>() as _);
    let node = Arc::new(Node::new(mb).unwrap());
    (node.clone(), node)
}
