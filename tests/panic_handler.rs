#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::{serial_print, serial_println, QemuExitCode, exit_qemu};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 14;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_print!("panic_handler... ");
	panic!(MESSAGE);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);
	loop {}
}

