// error: use of unstable library feature 'libc'
// help: add #![feature(libc)] to the crate attributes to enable
#![feature(libc)]

extern crate libc;
use libc::puts;
use std::ffi::CString;

fn main() {
	let sentence = "Merlin is the greatest magician!";
    let to_print = CString::new(sentence).unwrap();
    unsafe {
        puts(to_print.as_ptr());
    }
}
// Merlin is the greatest magician!