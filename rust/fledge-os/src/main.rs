#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fledge_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use fledge_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Windows is shit");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    fledge_os::test_panic_handler(info)
}
