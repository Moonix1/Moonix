pub mod port_manager;
pub mod serial;

use core::cell::RefCell;
use port_manager::{Port, PortManager};
use serial::Serial;

#[allow(unused_macros)]
macro_rules! sprint {
	($($arg:tt)*) => {
		#[allow(unused_unsafe)]
		unsafe {
			let mut sinks = crate::io::STDOUT_SINKS.borrow_mut();
			use core::fmt::Write as FmtWrite;
			if let Some(serial) = &mut sinks.serial {
                write!(serial, $($arg)*).expect("Failed to print to serial");
            }
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

pub struct StdoutSinksInner {
    pub serial: Option<Serial>,
}

pub struct StdoutSinks {
    inner: RefCell<StdoutSinksInner>,
}

impl core::ops::Deref for StdoutSinks {
    type Target = RefCell<StdoutSinksInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl StdoutSinks {
    const fn new() -> StdoutSinks {
        StdoutSinks {
            inner: RefCell::new(StdoutSinksInner {
                serial: None,
            }),
        }
    }
}

// RefCell is not sync, but we only have one thread...
unsafe impl Sync for StdoutSinks {}

pub static STDOUT_SINKS: StdoutSinks = StdoutSinks::new();

struct ExitPort {
    port: RefCell<Option<Port>>,
}

impl ExitPort {
    const fn new() -> ExitPort {
        ExitPort {
            port: RefCell::new(None),
        }
    }
}
// RefCell is not sync, but we only have one thread...
unsafe impl Sync for ExitPort {}

static EXIT_PORT: ExitPort = ExitPort::new();

pub fn init_stdio(port_manager: &mut PortManager) {
    let mut sinks = STDOUT_SINKS.inner.borrow_mut();
    sinks.serial = match Serial::new(port_manager) {
        Ok(v) => Some(v),
        Err(e) => {
            sprintln!("Failed to initialize serial output: {e}");
            None
        }
    }
}

pub fn init_late(port_manager: &mut PortManager) {
    const ISA_DEBUG_EXIT_PORT_NUM: u16 = 0xf4;
    let mut port = EXIT_PORT.port.borrow_mut();
    *port = Some(
        port_manager
            .request_port(ISA_DEBUG_EXIT_PORT_NUM)
            .expect("Failed to get exit port"),
    )
}

pub unsafe fn exit(code: u8) {
    let mut port = EXIT_PORT.port.borrow_mut();
    port.as_mut()
        .expect("exit port not initialized")
        .writeb(code);
}