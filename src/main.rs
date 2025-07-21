// Example main.rs for uxn11_rs_hooks
// Author: David Horner
// Demonstrates registering a callback and running a ROM

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void};
include!(concat!(env!("OUT_DIR"), "/uxn11_bindings.rs"));

extern "C" fn my_port_callback(port: u8, value: u8) {
    println!("[Rust] Callback: port=0x{:02x}, value=0x{:02x}", port, value);
}

fn main() {
    // Register the callback (cast to *mut c_void)
    unsafe {
        register_port_callback(my_port_callback as *mut c_void);
    }

    // Prepare arguments for uxn11_entry
    let args = vec![
        CString::new("uxn11").unwrap(),
        CString::new("example.rom").unwrap(),
    ];
    // Create mutable pointers for argv
    let mut argv: Vec<*mut c_char> = args.iter()
        .map(|s| s.as_ptr() as *mut c_char)
        .collect();
    let argc = argv.len() as c_int;

    // Call the C entry point
    let result = unsafe { uxn11_entry(argc, argv.as_mut_ptr()) };
    println!("uxn11_entry returned: {}", result);
}
