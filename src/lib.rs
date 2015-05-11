#![feature(asm, core, lang_items, no_std)]
#![no_std]

#[macro_use]
extern crate core;
extern crate rlibc;

pub use self::arch::init as arch_init;

mod arch;

mod console {
    pub use arch::serial::*;
}
mod unwind;
