// Copyright Dan Schatzberg, 2015. This file is part of Genesis.

// Genesis is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Genesis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with Genesis.  If not, see <http://www.gnu.org/licenses/>.
#![feature(asm, const_fn, lang_items, plugin, unique)]
#![plugin(clippy)]
#![no_std]

#![allow(empty_loop)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
//! The Genesis Microkernel.

#[cfg(not(test))]
extern crate core as std;
#[cfg(test)]
#[macro_use]
extern crate std;
#[macro_use]
extern crate bitflags;
extern crate fixedvec;
#[macro_use]
extern crate lazy_static;
extern crate multiboot;
extern crate rlibc;
extern crate spin;
extern crate x86;

pub use self::arch::init as arch_init;

#[macro_use]
mod log;

mod arch;
mod console {
    pub use arch::serial::*;
}
mod memory;
mod unwind;
