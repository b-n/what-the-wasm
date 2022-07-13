#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn sum_reduce(to: i64) -> i64 {
    let mut total: i64 = 0;
    for i in 0..=to {
        total += i
    }
    total
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
