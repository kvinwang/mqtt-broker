#[global_allocator]
static ALLOC: WasmTracingAllocator<wee_alloc::WeeAlloc> =
    WasmTracingAllocator { up: wee_alloc::WeeAlloc::INIT, enable_log: AtomicBool::new(true) };

use std::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct WasmTracingAllocator<A> {
    pub up: A,
    pub enable_log: AtomicBool,
}

unsafe impl<A> GlobalAlloc for WasmTracingAllocator<A>
where
    A: GlobalAlloc,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        if self.enable_log.swap(false, Ordering::Relaxed) {
            log::debug!("Alloc {size} bytes, align={align}");
            self.enable_log.store(true, Ordering::Relaxed);
        }
        self.up.alloc(layout)
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        self.up.dealloc(pointer, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.up.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, old_pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let align = layout.align();
        if self.enable_log.swap(false, Ordering::Relaxed) {
            log::debug!("Realloc {new_size} bytes, align={align}");
            self.enable_log.store(true, Ordering::Relaxed);
        }
        self.up.realloc(old_pointer, layout, new_size)
    }
}
