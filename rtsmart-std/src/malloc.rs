//! The basic memory alloc/free function uses rt-thread API

use crate::api::*;
use core::alloc::{GlobalAlloc, Layout};
use libc::c_void;
use crate::api::mem::{mem_alloc, mem_free};

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}

pub struct RttAlloc;

unsafe impl GlobalAlloc for RttAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        mem_alloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mem_free(ptr as *mut c_void)
    }
}
