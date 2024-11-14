use core::arch::asm;

fn inb(addr: u16) -> u8 {
    let mut ret = 0u8;
	unsafe {
		asm!(r#"
		.att_syntax
		in %dx, %al
		"#,
		in("dx") addr,
		out("al") ret);
	}

	ret
}

fn outb(addr: u16, val: u8) {
	unsafe {
		asm!(r#"
		.att_syntax
		out %al, %dx
		"#,
		in("dx") addr,
		in("al") val);
	}
}

pub mod serial;

#[allow(unused_macros)]
macro_rules! sprint {
	($($arg:tt)*) => {
		#[allow(unused_unsafe)]
		unsafe {
			use core::fmt::Write;
			$crate::io::serial::SERIAL.write_fmt(format_args!($($arg)*)).expect("Failed to print to serial");
		}
	};
}

#[allow(unused_macros)]
macro_rules! sprintln {
	($($arg:tt)*) => {
		sprint!($($arg)*);
		sprint!("\n");
	};
}