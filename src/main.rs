#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;
use vga_buffer::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().write_str("Hello again").unwrap();
	write!(WRITER.lock(), " some numbers: {}, {}", 42, 1.33).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
