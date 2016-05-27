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
use x86::irq::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ExceptionFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

/// Rust entry for all interrupts
#[no_mangle]
pub extern "C" fn interrupt_handler(num: usize, ef: u64) {
    let ef = ef as *mut ExceptionFrame;
    if num < EXCEPTIONS.len() {
        error!("Received Exception: {}", EXCEPTIONS[num].description);
    } else {
        error!("Recieved interrupt {}", num);
    }
    unsafe {
        error!("{:?}", *ef);
    }
}
