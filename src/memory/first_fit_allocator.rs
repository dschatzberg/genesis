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
use fixedvec::FixedVec;
use spin;
use super::{Frame, FrameAllocator, FrameRange, PAddr};

pub struct FirstFitAllocator<'a> {
    frames: &'a spin::Mutex<FixedVec<'a, FrameRange>>,
}

lazy_static! {
    static ref FRAMES: spin::Mutex<FixedVec<'static, FrameRange>> = {
        const FRAMES_SIZE: usize = 256;
        static mut FRAMES_MEM: [FrameRange; FRAMES_SIZE] = [FrameRange::new(
            Frame::down(PAddr::from_u64(0)), 0); FRAMES_SIZE];
        // Unsafe to take a mutable reference of a static.
        // We instantly store it behind a Mutex, so this is safe
        unsafe {
            spin::Mutex::new(FixedVec::new(&mut FRAMES_MEM))
        }
    };
    static ref ALLOCATOR: FirstFitAllocator<'static> = {
        FirstFitAllocator { frames: &FRAMES }
    };
}

impl FirstFitAllocator<'static> {
    pub fn get() -> &'static FirstFitAllocator<'static> {
        &ALLOCATOR
    }
}

impl<'a> FrameAllocator for FirstFitAllocator<'a> {
    fn allocate_manual(&self) -> Option<Frame> {
        self.allocate_range_manual(1).map(|range| range.start())
    }

    unsafe fn free_manual(&self, frame: Frame) {
        self.free_range_manual(FrameRange::new(frame, 1))
    }

    fn allocate_range_manual(&self, nframes: u64) -> Option<FrameRange> {
        let mut frames = self.frames.lock();
        frames.iter()
              .position(|range| range.nframes() >= nframes)
              .map(|index| {
                  let ret = FrameRange::new(frames[index].start(), nframes);
                  if frames[index].nframes() == nframes {
                      frames.remove(index);
                  } else {
                      frames[index].trim_front(nframes);
                  }
                  ret
              })
    }

    unsafe fn free_range_manual(&self, range: FrameRange) {
        let mut frames = self.frames.lock();
        let ind = {
            let slice = frames.as_slice();
            slice.binary_search_by(|r| r.partial_cmp(&range).unwrap())
                 .unwrap_err()
        };
        let prev_coalesce = if ind > 0 {
            if let Some(prev) = frames.get(ind - 1) {
                prev.end() == range.start()
            } else {
                false
            }
        } else {
            false
        };
        let next_coalesce = if ind < frames.len() {
            if let Some(next) = frames.get(ind) {
                range.end() == next.start()
            } else {
                false
            }
        } else {
            false
        };
        if !prev_coalesce && !next_coalesce {
            if frames.insert(ind, range).is_err() {
                warn!("No space to store freed range.\
                       It will be forgetten: {:?}",
                      range)
            }
        } else if prev_coalesce && !next_coalesce {
            let mut prev = frames.get_mut(ind - 1).unwrap();
            prev.push_back(range.nframes());
        } else if !prev_coalesce && next_coalesce {
            let mut next = frames.get_mut(ind).unwrap();
            next.push_front(range.nframes());
        } else {
            let nframes = frames.get(ind).unwrap().nframes() + range.nframes();
            {
                let mut prev = frames.get_mut(ind - 1).unwrap();
                prev.push_back(nframes);
            }
            frames.remove(ind);
        }
    }
}

#[cfg(test)]
mod test {
    use super::FirstFitAllocator;
    use fixedvec::FixedVec;
    use memory::{Frame, FrameAllocator, FrameRange, PAddr};
    use spin;

    #[test]
    fn test_get() {
        FirstFitAllocator::get();
    }

    const fn create_range(start_page: u64, nframes: u64) -> FrameRange {
        FrameRange::new(Frame { num: start_page }, nframes)
    }

    #[test]
    fn test_simple() {
        let mut space = [create_range(0, 0); 256];
        let frames = spin::Mutex::new(FixedVec::new(&mut space));
        let allocator = FirstFitAllocator { frames: &frames };
        let r = create_range(0, 1);
        unsafe { allocator.free_range_manual(r) };
        assert_eq!(allocator.allocate_range_manual(1).unwrap(), r);
    }

    #[test]
    fn test_prev_coalesce() {
        let mut space = [create_range(0, 0); 256];
        let frames = spin::Mutex::new(FixedVec::new(&mut space));
        let allocator = FirstFitAllocator { frames: &frames };
        unsafe { allocator.free_range_manual(create_range(0, 1)) };
        unsafe { allocator.free_range_manual(create_range(1, 1)) };
        assert_eq!(allocator.allocate_range_manual(2).unwrap(),
                   create_range(0, 2));
    }

    #[test]
    fn test_next_coalesce() {
        let mut space = [create_range(0, 0); 256];
        let frames = spin::Mutex::new(FixedVec::new(&mut space));
        let allocator = FirstFitAllocator { frames: &frames };
        unsafe { allocator.free_range_manual(create_range(1, 1)) };
        unsafe { allocator.free_range_manual(create_range(0, 1)) };
        assert_eq!(allocator.allocate_range_manual(2).unwrap(),
                   create_range(0, 2));
    }

    #[test]
    fn test_both_coalesce() {
        let mut space = [create_range(0, 0); 256];
        let frames = spin::Mutex::new(FixedVec::new(&mut space));
        let allocator = FirstFitAllocator { frames: &frames };
        unsafe { allocator.free_range_manual(create_range(0, 1)) };
        unsafe { allocator.free_range_manual(create_range(2, 1)) };
        unsafe { allocator.free_range_manual(create_range(1, 1)) };
        assert_eq!(allocator.allocate_range_manual(3).unwrap(),
                   create_range(0, 3));
    }
}
