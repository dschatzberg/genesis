#![feature(core, lang_items, no_std)]
#![no_std]

extern crate core;

pub use self::arch::init as arch_init;

mod arch;
mod unwind;
