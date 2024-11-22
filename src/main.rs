#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;

use core::panic::PanicInfo;
use rusty_os::{print, println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rusty_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    rusty_os::init();

    #[cfg(test)]
    test_main();

    println!("This didn't reboot!");
    rusty_os::hlt_loop();
}
