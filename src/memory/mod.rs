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

use core::ops::{Deref, DerefMut};
pub use ::arch::mem::*;
pub mod first_fit_allocator;

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
