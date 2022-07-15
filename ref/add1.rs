#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn add1(i: i32) -> i32 {
    i + 1
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
