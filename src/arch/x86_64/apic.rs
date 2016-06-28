// Copyright Dan Schatzberg, 2016. This file is part of Genesis.

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
#![allow(non_camel_case_types)]

use memory::*;
use core::ptr;
use x86::msr::*;

#[allow(dead_code)]
enum Reg {
    ID = 0x20,
    LVR = 0x30,
    TPR = 0x80,
    APR = 0x90,
    PPR = 0xa0,
    EOR = 0xb0,
    RRR = 0xc0,
    LDR = 0xd0,
    DFR = 0xe0,
    SPIV = 0xf0,
    ISR = 0x100,
    TMR = 0x180,
    IRR = 0x200,
    ESR = 0x280,
    ICR = 0x300,
    ICR2 = 0x310,
    LVTT = 0x320,
    LVTTHMR = 0x330,
    LVTPC = 0x340,
    LVT0 = 0x350,
    LVT1 = 0x360,
    LVTERR = 0x370,
    TMICT = 0x380,
    TMCCT = 0x390,
    TMDCR = 0x3e0,
    SELF_IPI = 0x3f0,
}

const BASE_GLOBAL_ENABLE: u64 = 1 << 11;

const SPIV_SOFTWARE_ENABLE: u32 = 1 << 8;

pub struct Apic {
    base_addr: VAddr,
}

impl Apic {
    pub unsafe fn init<Allocator>(page_table: &mut PageTable,
                                  allocator: &Allocator)
        where Allocator: FrameAllocator
    {
        let apic_base = rdmsr(APIC_BASE);
        let apic_paddr = PAddr::from_u64(apic_base & !0xfff);
        let apic_vaddr = phys_to_virt(apic_paddr);
        let frame = Frame::down(apic_paddr);
        let page = Page::down(apic_vaddr);
        page_table.map_device(page,
                              frame,
                              allocator,
                              |f: Frame| frame_to_slice(f));
        wrmsr(APIC_BASE, apic_base | BASE_GLOBAL_ENABLE);
        let mut apic = Apic { base_addr: apic_vaddr };
        apic.write(Reg::SPIV, SPIV_SOFTWARE_ENABLE);
    }

    fn get_ptr(&self, reg: Reg) -> *mut u32 {
        unsafe {
            (self.base_addr.as_usize() as *mut u8)
                .offset(reg as isize) as *mut u32
        }
    }

    fn write(&mut self, reg: Reg, value: u32) {
        unsafe {
            ptr::write_volatile(self.get_ptr(reg), value);
        };
    }
}
