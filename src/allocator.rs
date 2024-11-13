use core::{alloc::{
	GlobalAlloc, Layout,
}, sync::atomic::{AtomicPtr, Ordering}};

use boot::MemoryType;
use log::info;
use uefi::{mem::memory_map::{MemoryMap, MemoryMapOwned}, prelude::*, println};

#[repr(C, packed)]
struct FreeSegment {
	size: usize,
	next_segment: *mut FreeSegment,
}

impl FreeSegment {
	fn get_start(&self) -> *mut u8 {
		unsafe { (self as *const FreeSegment).add(1) as *mut u8 }
	}

	fn get_end(&self) -> *mut u8 {
		unsafe { self.get_start().add(self.size) }
	}
}

#[repr(C, packed)]
struct UsedSegment {
	size: usize,
	padding: [u8 ; 8],
}

pub struct Allocator {
	first_free: AtomicPtr<FreeSegment>
}

impl Allocator {
	pub const fn new() -> Self {
		Self { first_free: AtomicPtr::new(core::ptr::null_mut()) }
	}

	pub fn init(&self, mmap: MemoryMapOwned) {
		assert_eq!(core::mem::size_of::<UsedSegment>(), core::mem::size_of::<FreeSegment>());

		let big_block = mmap.entries().find(|entry| {
			entry.ty == MemoryType::LOADER_DATA
		}).expect("Failed to find big block of RAM!");

		let big_block_start = big_block.phys_start as *mut u8;
		let big_block_size = big_block.page_count * 4096;

		let segment_start = unsafe { big_block_start.add(core::mem::size_of::<FreeSegment>()) };

		let segment_size = big_block_size as usize - core::mem::size_of::<FreeSegment>();

		let segment = segment_start as *mut FreeSegment;
		unsafe {
			(*segment).size = segment_size;
			(*segment).next_segment = core::ptr::null_mut();
		}

		self.first_free.store(segment, Ordering::Relaxed);
	}
}

unsafe fn get_header_ptr(segment: &FreeSegment, layout: &Layout) -> Option<*mut u8> {
	let segment_start = segment.get_start();
	let segment_end = segment.get_end();

	let mut ptr = segment_end.sub(layout.size());
	ptr = ptr.sub((ptr as usize) % layout.align());
	ptr = ptr.sub(core::mem::size_of::<UsedSegment>());
	
	if ptr < segment_start {
		return None
	}

	Some(ptr)
}

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut free_block_it = self.first_free.load(Ordering::Relaxed);

		while !free_block_it.is_null() {
			let header_ptr = get_header_ptr(&*free_block_it, &layout);
			let header_ptr = match header_ptr {
				Some(v) => v,
				None => {
					free_block_it = (*free_block_it).next_segment;
					continue;
				}
			};

			let segment_end = (*free_block_it).get_end();

			let new_size = header_ptr.offset_from((*free_block_it).get_start());
			let new_size = new_size.try_into().expect("expected valid usize for new fragment size");
			(*free_block_it).size = new_size;

			let header_ptr = header_ptr as *mut UsedSegment;
			(*header_ptr).size = segment_end.offset_from(header_ptr.add(1) as *const u8)
				.try_into()
				.expect("expected segment end offset to be castable to usize");

			return header_ptr.add(1) as *mut u8;
		}

		panic!("Failed to allocate!");
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unimplemented!()
	}
}