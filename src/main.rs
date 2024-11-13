#![no_main]
#![no_std]

use core::{mem, num::NonZero, panic::PanicInfo, ptr::NonNull};

extern crate uefi;
extern crate alloc;

mod allocator;

use alloc::vec::*;
use alloc::vec;
use boot::MemoryType;
use uefi::{mem::memory_map::MemoryMap, prelude::*};
use log::*;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator::new();

fn setup_stack(stack_size: usize) -> (Result<NonNull<u8>, Status>, NonNull<u8>) {
	let aligned_size = (stack_size + 4095) & !4095;
	let stack_ptr =
		boot::allocate_pages(
			boot::AllocateType::AnyPages,
			MemoryType::LOADER_DATA,
			aligned_size / 4096
		).map_err(|e| e.status());

	let stack_end = unsafe { stack_ptr.unwrap().add(aligned_size) };

	(Ok(stack_ptr.unwrap()), stack_end)
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

	// Allocate 1MB of stack space
	let (stack_ptr, stack_end) = setup_stack(1 * 1024 * 1024);
	
	let mmap = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
	let mut total_len = 0;
	for i in 0..mmap.len() {
		let desc = &mmap[i];
		let len = desc.page_count * 4096;
		total_len += len;
		info!("size {}, len: {}, addr: {:#X}", size_of_val(desc), len, desc.phys_start);
	}

	info!("{}MB", total_len / 1024 / 1024);
	info!("stack start: {:#X}", stack_ptr.unwrap().addr());
	info!("stack end: {:#X}", stack_end.addr());
	
	ALLOC.init(mmap);

	let mut v: Vec<i64> = Vec::new();

	for i in 0..100 {
		v.push(i);
	}

	info!("{:?}", v);

    trace!("Hello world!");
    
	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	error!("panic: {:?}", info);

	loop {}
}