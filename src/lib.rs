//! Cortex-M processor
//!
//! This crate provides functionality common to all Cortex-M processors:
//!
//! - Some assembly instructions
//! - Register definition of core peripherals
//! - Exception vector
//!   - This crate provides a reset handler that initializes RAM memory and then calls an external
//!     `start` function, which another crate must provide.
//!   - All the other exceptions are weakly linked to an `abort` function, this `abort` function
//!     puts the processor in debug mode (`bkpt`), and then busy waits.
//! - Lang items like `panic_fmt` has been stubbed with the `abort` function

#![feature(asm)]
#![feature(core)]
#![feature(lang_items)]
#![feature(no_std)]
#![no_std]

extern crate core;
extern crate volatile;

#[macro_use]
extern crate reg;

mod mem;
mod lang_items;

pub mod asm;
pub mod exception;
pub mod nvic;
pub mod peripheral;
pub mod stubs;

/// Reset handler
#[no_mangle]
pub unsafe extern fn reset() {
    extern {
        fn start();
    }

    mem::initialize_data();
    mem::zero_bss();

    start();
}

/// Terminate program
#[no_mangle]
pub extern fn abort() -> ! {
    asm::bkpt();

    loop {}
}
