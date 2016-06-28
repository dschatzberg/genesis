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
use x86::msr::*;
use x86::segmentation::*;
use x86::rflags::*;

pub fn init() {
    let call_cs = (SegmentSelector::new(0x8) | RPL_0 | TI_GDT).bits() as u64;
    let ret_cs = (SegmentSelector::new(0x10) | RPL_3 | TI_GDT).bits() as u64;
    unsafe {
        let star = rdmsr(IA32_STAR);
        wrmsr(IA32_STAR, star | call_cs << 32 | ret_cs << 48);
    }
    extern "C" {
        static syscall_entry: u8;
    }
    let syscall_ptr: *const u8 = &syscall_entry;
    unsafe {
        wrmsr(IA32_LSTAR, syscall_ptr as usize as u64);
        wrmsr(IA32_FMASK, !RFLAGS_A1.bits());
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | 0x1);
    }
}
