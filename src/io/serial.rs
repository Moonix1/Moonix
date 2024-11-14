use crate::io::{inb, outb};

const PORT: u16 = 0x3F8;

fn is_transmit_empty() -> bool {
	inb(PORT + 5) & 0x20 == 0
}

pub static mut SERIAL: Serial = Serial {};

fn write_serial(a: u8) {
	while is_transmit_empty() {}

	outb(PORT, a);
}

#[derive(Debug)]
pub struct SerialInitFault;

pub struct Serial {}

impl Serial {
	pub fn init() -> Result<(), SerialInitFault> {
		outb(PORT + 1, 0x00); // Disable all interrupts
		outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
		outb(PORT + 0, 0x03); // Set divisor to 3 (low byte) for 38400 baud
		outb(PORT + 1, 0x00); // Set divisor high byte to 0
		outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
		outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
		outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
		outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
		outb(PORT + 0, 0xAE); // Send byte 0xAE and check if it is received

		// Check if the serial port is faulty (i.e., if it didn't return 0xAE)
		if inb(PORT + 0) != 0xAE {
			return Err(SerialInitFault);
		}

		// Set normal operation mode (disable loopback mode)
		outb(PORT + 4, 0x0F);

		Ok(())
	}
}

impl core::fmt::Write for Serial {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for b in s.as_bytes() {
			write_serial(*b);
		}

		Ok(())
	}
}