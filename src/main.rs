#![no_main]
#![no_std]

use core::{panic::PanicInfo, ptr::NonNull};

extern crate uefi;
extern crate alloc;

mod allocator;

#[macro_use]
mod io;

use boot::MemoryType;
use io::serial::Serial;
use uefi::prelude::*;
use log::*;

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
unsafe fn main() -> Status {
    uefi::helpers::init().unwrap();

	// Allocate 1MB of stack space
	let (stack_ptr, stack_end) =
		setup_stack(1 * 1024 * 1024);
	let mmap = boot::memory_map(MemoryType::LOADER_DATA).unwrap();

	allocator::init(mmap);
    let mut port_manager = io::port_manager::PortManager::new();
    io::init_stdio(&mut port_manager);
    io::init_late(&mut port_manager);

	sprintln!("Hello there!");

	loop {}

	Status::SUCCESS
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	error!("panic: {:?}", info);

	loop {}
}