#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
#[no_mangle]
fn main() -> i32 {
	let a = 1;
	let b = 2;
	let sum_result = add(a, b);
	println!("Sum: {}", sum_result);
	0
}
fn add(x: i32, y: i32) -> i32 {
	x + y
}