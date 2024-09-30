#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use spin::Mutex;

pub struct Node {
    size: usize,
    next: AtomicPtr<Node>, // Use AtomicPtr for thread safety
}

pub struct BumpAllocator {
    start: usize,
    end: usize,
    current: usize,
    free_list: AtomicPtr<Node>, // Use AtomicPtr for free list
    total_allocated: usize,
}

impl BumpAllocator {
    pub const fn new(start: usize, end: usize) -> Self {
        BumpAllocator {
            start,
            end,
            current: start,
            free_list: AtomicPtr::new(ptr::null_mut()),
            total_allocated: 0,
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        if !self.free_list.load(Ordering::SeqCst).is_null() {
            // Try to find a free block that fits
            let mut current_node = self.free_list.load(Ordering::SeqCst);

            while !current_node.is_null() {
                let node_size = (*current_node).size;

                if node_size >= size {
                    // Remove from free list and return
                    self.free_list.store((*current_node).next.load(Ordering::SeqCst), Ordering::SeqCst);
                    self.total_allocated += size;
                    return current_node as *mut u8; // Return the node's pointer
                }

                current_node = (*current_node).next.load(Ordering::SeqCst);
            }
        }

        // No suitable block found; fallback to bump allocation
        let new_start = self.current;
        let new_end = new_start + size;

        if new_end <= self.end {
            self.current = new_end;
            self.total_allocated += size;
            new_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let new_node = ptr as *mut Node;

        (*new_node) = Node {
            size,
            next: AtomicPtr::new(self.free_list.load(Ordering::SeqCst)),
        };

        self.free_list.store(new_node, Ordering::SeqCst); // Add to the free list
    }

    pub fn available_memory(&self) -> usize {
        self.end - self.current
    }

    pub fn heap_size(&self) -> usize {
        self.end - self.start
    }

    pub fn used_memory(&self) -> usize {
        self.total_allocated
    }

    pub fn remaining_memory(&self) -> usize {
        self.heap_size() - self.total_allocated
    }
}

// Global allocator setup
pub static GLOBAL_ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new(0x90000, 0xA0000));

pub struct GlobalAllocatorWrapper;

unsafe impl GlobalAlloc for GlobalAllocatorWrapper {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = GLOBAL_ALLOCATOR.lock();
        allocator.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = GLOBAL_ALLOCATOR.lock();
        allocator.dealloc(ptr, layout);
    }
}
