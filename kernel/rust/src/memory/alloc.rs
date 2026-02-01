use core::alloc::{GlobalAlloc, Layout};
use spin::Mutex;

const HEAP_START: usize = 0x200000;
const HEAP_SIZE: usize = 0x400000;

struct BumpAllocator {
    next: usize,
    end: usize,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self {
            next: HEAP_START,
            end: HEAP_START + HEAP_SIZE,
        }
    }
    
    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        
        let start = (self.next + align - 1) & !(align - 1);
        let end = start + size;
        
        if end > self.end {
            return core::ptr::null_mut();
        }
        
        self.next = end;
        start as *mut u8
    }
}

#[global_allocator]
static ALLOCATOR: LockedAllocator = LockedAllocator(Mutex::new(BumpAllocator::new()));

struct LockedAllocator(Mutex<BumpAllocator>);

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

pub fn init() {}
