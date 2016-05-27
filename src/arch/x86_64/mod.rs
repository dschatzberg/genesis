// This file is part of Genesis.

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
pub use super::x86::serial;

/// Loading and manipulating the x86_64 Global Descriptor Table
pub mod gdt;
/// Loading and manipulating the x86_64 Interrupt Descriptor Table
pub mod idt;
/// Architecture specific boot code.
pub mod init;
/// Memory management routines
pub mod mem;
