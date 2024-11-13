#![no_main]
#![no_std]

use core::{mem, panic::PanicInfo};

extern crate uefi;
extern crate alloc;

mod allocator;

use alloc::vec::Vec;
use boot::MemoryType;
use uefi::{mem::memory_map::MemoryMap, prelude::*};
use log::*;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator::new();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
	let mmap = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
	let mut total_len = 0;
	for i in 0..mmap.len() {
		let desc = &mmap[i];
		let len = desc.page_count * 4096;
		total_len += len;
		info!("size {}, len: {}, addr: {}", size_of_val(desc), len, desc.phys_start);
	}

	info!("{}MB", total_len / 1024 / 1024);
	
	ALLOC.init(mmap);


    trace!("Hello world!");
    boot::stall(10_000_000);
    Status::SUCCESS
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	error!("panic: {:?}", info);

	loop {}
}