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

use core::cmp::Ordering;
use core::ops::{Add, Deref, DerefMut, Sub};
pub use ::arch::mem::*;
pub mod first_fit_allocator;

/// A physical frame (page)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    num: u64,
}

impl Frame {
    /// Returns the starting address of the `Frame`
    pub const fn start_address(&self) -> PAddr {
        PAddr::from_u64(self.num << BASE_PAGE_SHIFT)
    }

    /// Round `addr` up to the closest `Frame`
    pub const fn up(addr: PAddr) -> Frame {
        Frame { num: (addr.as_u64() + BASE_PAGE_SIZE - 1) >> BASE_PAGE_SHIFT }
    }

    /// Round `addr` down to the closest `Frame`
    pub const fn down(addr: PAddr) -> Frame {
        Frame { num: addr.as_u64() >> BASE_PAGE_SHIFT }
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

/// A virtual page
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    num: usize,
}

impl Page {
    /// Returns the starting address of the `Page`
    pub const fn start_address(&self) -> VAddr {
        VAddr::from_usize(self.num << BASE_PAGE_SHIFT)
    }

    /// Round `addr` up to the closest `Page`
    pub const fn up(addr: VAddr) -> Page {
        Page {
            num: (addr.as_usize() + (BASE_PAGE_SIZE as usize) - 1) >>
                 BASE_PAGE_SHIFT,
        }
    }

    /// Round `addr` down to the closest `Page`
    pub const fn down(addr: VAddr) -> Page {
        Page { num: addr.as_usize() >> BASE_PAGE_SHIFT }
    }
}

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page { num: self.num + rhs }
    }
}

impl Sub<usize> for Page {
    type Output = Page;

    fn sub(self, rhs: usize) -> Page {
        Page { num: self.num - rhs }
    }
}

impl Sub<Page> for Page {
    type Output = usize;

    fn sub(self, rhs: Page) -> usize {
        self.num - rhs.num
    }
}

pub struct FrameHandle<'a, T: 'a + FrameAllocator>(Frame, &'a T);

impl<'a, T: 'a + FrameAllocator> Deref for FrameHandle<'a, T> {
    type Target = Frame;

    fn deref(&self) -> &Frame {
        &self.0
    }
}

impl<'a, T: 'a + FrameAllocator> DerefMut for FrameHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Frame {
        &mut self.0
    }
}

impl<'a, T: 'a + FrameAllocator> Drop for FrameHandle<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.1.free_manual(self.0);
        }
    }
}

pub struct FrameRangeHandle<'a, T: 'a + FrameAllocator>(FrameRange, &'a T);

impl<'a, T: 'a + FrameAllocator> Deref for FrameRangeHandle<'a, T> {
    type Target = FrameRange;

    fn deref(&self) -> &FrameRange {
        &self.0
    }
}

impl<'a, T: 'a + FrameAllocator> DerefMut for FrameRangeHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut FrameRange {
        &mut self.0
    }
}

impl<'a, T: 'a + FrameAllocator> Drop for FrameRangeHandle<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.1.free_range_manual(self.0);
        }
    }
}

pub trait FrameAllocator : Sync + Sized {
    fn allocate_manual(&self) -> Option<Frame>;
    unsafe fn free_manual(&self, Frame);

    fn allocate(&self) -> Option<FrameHandle<Self>> {
        let opt_range = self.allocate_manual();
        opt_range.map(|range| FrameHandle(range, self))
    }

    fn allocate_range_manual(&self, nframes: u64) -> Option<FrameRange>;
    unsafe fn free_range_manual(&self, FrameRange);

    fn allocate_range(&self, nframes: u64) -> Option<FrameRangeHandle<Self>> {
        let opt_range = self.allocate_range_manual(nframes);
        opt_range.map(|range| FrameRangeHandle(range, self))
    }
}
