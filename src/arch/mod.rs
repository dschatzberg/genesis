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
#[cfg(any(target_arch = "x86",target_arch = "x86_64"))]
pub use self::x86::serial;

#[cfg(any(target_arch = "x86",target_arch = "x86_64"))]
mod x86;

#[cfg(target_arch = "x86_64")]
/// x86_64 specific interfaces
pub mod x86_64;

/// Architecture-specific memory management definitions
pub mod mem {
    #[cfg(any(target_arch = "x86_64"))]
    pub use arch::x86_64::mem::*;
}

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::arch_init;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::interrupt_handler;
