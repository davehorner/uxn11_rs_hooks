// uxn11_rs_hooks - Rust hooks for Uxn11
// Author: David Horner
// Created: July 2025

// uxn11_rs_hooks - Rust hooks for Uxn11
// Author: David Horner
// Created: July 2025

use std::os::raw::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

// Type for the port callback
pub type PortCallback = extern "C" fn(port: u8, value: u8);

// Static atomic pointer to the callback
static PORT_CALLBACK: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Register a callback to be called for every port event.
/// Pass Some(function) to set, or None to clear.
#[no_mangle]
pub extern "C" fn register_port_callback(cb: Option<PortCallback>) {
    let ptr = cb.map_or(std::ptr::null_mut(), |f| f as *mut c_void);
    PORT_CALLBACK.store(ptr, Ordering::SeqCst);
}

/// Called from C for every port event
#[no_mangle]
pub extern "C" fn rust_deo_all_ports_hook(port: u8, value: u8) {
    let cb = PORT_CALLBACK.load(Ordering::SeqCst);
    if !cb.is_null() {
        let f: PortCallback = unsafe { std::mem::transmute(cb) };
        f(port, value);
    }
}
