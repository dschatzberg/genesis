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
pub use x86::paging::*;
use memory::{FrameAllocator, Page};

use core::cmp::Ordering;
use core::mem;
use core::ops::{Add, Sub};
use core::ptr::Unique;

pub const PHYS_MAP: usize = 0xFFFF_FF80_0000_0000;
pub const PHYS_LIMIT: u64 = 0x80_0000_0000;

pub fn phys_to_virt(p: PAddr) -> VAddr {
    debug_assert!(p.as_u64() < PHYS_LIMIT);
    VAddr::from_usize(p.as_u64() as usize + PHYS_MAP)
}

pub type PageSlice = [u8; PAGE_SIZE as usize];

pub unsafe fn frame_to_slice<'a>(frame: Frame) -> &'a mut PageSlice {
    mem::transmute(phys_to_virt(frame.start_address()).as_usize())
}

/// A physical frame (page)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    num: u64,
}

pub const PAGE_SHIFT: u8 = 12;
pub const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
impl Frame {
    /// Returns the starting address of the `Frame`
    pub const fn start_address(&self) -> PAddr {
        PAddr::from_u64(self.num << PAGE_SHIFT)
    }

    /// Round `addr` up to the closest `Frame`
    pub const fn up(addr: PAddr) -> Frame {
        Frame { num: (addr.as_u64() + PAGE_SIZE - 1) >> PAGE_SHIFT }
    }

    /// Round `addr` down to the closest `Frame`
    pub const fn down(addr: PAddr) -> Frame {
        Frame { num: addr.as_u64() >> PAGE_SHIFT }
    }
}

impl Add<u64> for Frame {
    type Output = Frame;

    fn add(self, rhs: u64) -> Frame {
        Frame { num: self.num + rhs }
    }
}

impl Sub<u64> for Frame {
    type Output = Frame;

    fn sub(self, rhs: u64) -> Frame {
        Frame { num: self.num - rhs }
    }
}

impl Sub<Frame> for Frame {
    type Output = u64;

    fn sub(self, rhs: Frame) -> u64 {
        self.num - rhs.num
    }
}

/// A contiguous range of `Frame`s (always left-inclusive)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FrameRange {
    lower: Frame,
    upper: Frame,
}

impl FrameRange {
    /// Construct a `FrameRange` [`lower`, `upper`)
    pub const fn new(lower: Frame, upper: Frame) -> FrameRange {
        FrameRange {
            lower: lower,
            upper: upper,
        }
    }

    /// Get the first `Frame`
    pub const fn lower(&self) -> Frame {
        self.lower
    }

    /// Get the last `Frame` (after the range)
    pub const fn upper(&self) -> Frame {
        self.upper
    }

    /// Get the number of `Frame`s
    pub const fn nframes(&self) -> u64 {
        self.upper.num - self.lower.num
    }

    /// Trim `nframes` off the front of the range
    pub fn trim_front(&mut self, nframes: u64) {
        assert!(self.nframes() > nframes);
        self.lower = self.lower + nframes;
    }

    /// Trim `nframes` off the back of the range
    pub fn trim_back(&mut self, nframes: u64) {
        assert!(self.nframes() > nframes);
        self.upper = self.upper - nframes;
    }

    /// add `nframes` to the front of the range
    pub fn push_front(&mut self, nframes: u64) {
        self.lower = self.lower - nframes;
    }

    /// add `nframes` to the back of the range
    pub fn push_back(&mut self, nframes: u64) {
        self.upper = self.upper + nframes;
    }
}

impl PartialOrd for FrameRange {
    fn partial_cmp(&self, other: &FrameRange) -> Option<Ordering> {
        if self.lower() == other.lower() && self.upper() == other.upper() {
            Some(Ordering::Equal)
        } else if self.upper() <= other.lower() {
            Some(Ordering::Less)
        } else if self.lower() >= other.upper() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

pub struct PageTable {
    table: Unique<PML4>,
}

impl PageTable {
    pub unsafe fn new(table: *mut PML4) -> PageTable {
        PageTable { table: Unique::new(table) }
    }

    pub fn get(&self) -> &PML4 {
        unsafe { self.table.get() }
    }

    pub fn get_mut(&mut self) -> &mut PML4 {
        unsafe { self.table.get_mut() }
    }

    pub fn map<'a, Allocator, F>(&'a mut self,
                                 page: Page,
                                 frame: Frame,
                                 flags: PTEntry,
                                 allocator: &Allocator,
                                 f: F)
        where Allocator: FrameAllocator,
              F: Fn(Frame) -> &'a mut [u8; PAGE_SIZE as usize]
    {
        let pml4 = self.get_mut();
        let pml4_idx = pml4_index(page.start_address());
        if pml4[pml4_idx].is_empty() {
            let frame = allocator.allocate_manual()
                .expect("Could not allocate frame for PDPT");
            for b in f(frame).iter_mut() {
                *b = 0;
            }
            pml4[pml4_idx] = PML4Entry::new(frame.start_address(),
                                            PML4_P | PML4_RW);
        }
        let pdpt: &mut PDPT = unsafe {
            mem::transmute(f(Frame::down(pml4[pml4_idx].get_address())))
        };
        let pdpt_idx = pdpt_index(page.start_address());
        if pdpt[pdpt_idx].is_empty() {
            let frame = allocator.allocate_manual()
                .expect("Could not allocate frame for PD");
            for b in f(frame).iter_mut() {
                *b = 0;
            }
            pdpt[pdpt_idx] = PDPTEntry::new(frame.start_address(),
                                            PDPT_P | PDPT_RW);
        }

        let pd: &mut PD = unsafe {
            mem::transmute(f(Frame::down(pdpt[pdpt_idx].get_address())))
        };
        let pd_idx = pd_index(page.start_address());
        if pd[pd_idx].is_empty() {
            let frame = allocator.allocate_manual()
                .expect("Could not allocate frame for PT");
            for b in f(frame).iter_mut() {
                *b = 0;
            }
            pd[pd_idx] = PDEntry::new(frame.start_address(), PD_P | PD_RW);
        }

        let pt: &mut PT = unsafe {
            mem::transmute(f(Frame::down(pd[pd_idx].get_address())))
        };
        let pt_idx = pt_index(page.start_address());
        assert!(pt[pt_idx].is_empty());
        pt[pt_idx] = PTEntry::new(frame.start_address(), flags);
    }

    pub fn map_device<'a, Allocator, F>(&'a mut self,
                                        page: Page,
                                        frame: Frame,
                                        allocator: &Allocator,
                                        f: F)
        where Allocator: FrameAllocator,
              F: Fn(Frame) -> &'a mut [u8; PAGE_SIZE as usize]
    {
        self.map(page, frame, PT_P | PT_G | PT_RW | PT_PCD, allocator, f)
    }
}
