#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[my_test] Hello from 23301098!");
    println!("Test my_test OK!");
    0
}
