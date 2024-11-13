// WIP

use core::{alloc::{
	GlobalAlloc, Layout,
}, sync::atomic::AtomicPtr};

use uefi::mem::memory_map::MemoryMapOwned;

struct FreeSegment {
	size: usize,
	offset: usize,
	next_segment: *mut FreeSegment,
}

pub struct Allocator {
	first_free: AtomicPtr<FreeSegment>
}

impl Allocator {
	pub const fn new() -> Self {
		Self { first_free: AtomicPtr::new(core::ptr::null_mut()) }
	}

	pub fn init(&self, mmap: MemoryMapOwned) {

	}
}

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unimplemented!()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unimplemented!()
	}
}