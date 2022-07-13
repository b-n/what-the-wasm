#![no_std]

extern crate std;

use std::io;
use std::io::prelude::*;
use std::string::ToString;
use std::vec::Vec;

const ARR_LEN: usize = 10;

pub fn main() {
    let mut array: Vec<usize> = Vec::with_capacity(ARR_LEN);
    array.resize(ARR_LEN, 0);

    for (i, v) in array.iter_mut().enumerate() {
        *v = i * 2;
    }

    for v in array.iter() {
        io::stdout().write(&v.to_string().as_bytes()).unwrap();
        io::stdout().write(&[10]).unwrap();
    }
}
