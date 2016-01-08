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
use core::u32;
use core::mem;
use x86::dtables::*;
use x86::segmentation::*;
use x86::task::*;

lazy_static! {
    static ref TSS: TaskStateSegment = TaskStateSegment::new();

    static ref GDT: [SegmentDescriptor; 7] = {
        let tss_addr = {
            let ptr: *const _ = &*TSS;
            ptr as u64
        };

        [SegmentDescriptor::empty(), // NULL Descriptor
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_C_ER | DESC_S | DESC_DPL0 | DESC_P | DESC_L | DESC_G,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_D_RW | DESC_S | DESC_DPL0 | DESC_P,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_D_RW | DESC_S | DESC_DPL3 | DESC_P,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_C_ER | DESC_S | DESC_DPL3 | DESC_P | DESC_L | DESC_G,
         {
             SegmentDescriptor::new((tss_addr & 0xFFFF_FFFF) as u32,
                                    mem::size_of::<TaskStateSegment>() as u32)
                 | TYPE_SYS_TSS_AVAILABLE | DESC_S | DESC_DPL0 | DESC_P
         },
         unsafe { mem::transmute(tss_addr >> 32) }
         ]
    };
}

pub fn reset() {
    let gdt_ptr = {
        let gdtp: *const _ = &*GDT;
        let gdt_size = (mem::size_of::<SegmentDescriptor>() * GDT.len() -
                        1) as u16;
        DescriptorTablePointer {
            limit: gdt_size,
            base: gdtp as u64,
        }
    };
    unsafe {
        lgdt(&gdt_ptr);
        load_cs(SegmentSelector::new(1));
        load_ss(SegmentSelector::new(2));
    }

}
