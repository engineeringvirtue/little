#![no_main]
#![no_std]

#![feature(core_intrinsics)]

extern crate little;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

// mod platform;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	let _ = hprintln!("{}", info);
	unsafe { core::intrinsics::abort() }
}

struct CoreData {
	weather: u8,
	date: u16,
	hour: u8,
	minute: u8
}

#[entry]
fn main() -> ! {
	hprintln!("hewwo wurld").unwrap();
	loop {}
}
