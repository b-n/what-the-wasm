#![no_main]

#[no_mangle]
pub extern "C" fn sum_reduce(to: i64) -> i64 {
    let array = vec![0,1,2];
    let mut total: i64 = 0;
    for i in 0..=to {
        total += i
    }
    total
}
