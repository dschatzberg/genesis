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
use super::serial;
use core::mem::transmute;
use core::slice;
use fixedvec::FixedVec;
use memory::{Frame, FrameAllocator, FrameRange, PAddr};
use memory::first_fit_allocator::FirstFitAllocator;
use multiboot::{self, MemoryType, Multiboot};
use spin;

unsafe fn early_paddr_to_slice<'a>(p: multiboot::PAddr,
                                   sz: usize)
                                   -> Option<&'a [u8]> {
    if (sz < (1 << 30)) && ((p + sz as u64) < (1 << 30)) {
        let ptr = transmute(p + 0xFFFFFFFFC0000000);
        Some(slice::from_raw_parts(ptr, sz))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct MemoryRegion {
    start: PAddr,
    end: PAddr,
}

impl MemoryRegion {
    const fn new(start: PAddr, end: PAddr) -> MemoryRegion {
        MemoryRegion {
            start: start,
            end: end,
        }
    }

    fn trim_below(&mut self, addr: PAddr) {
        if self.start < addr && self.end > addr {
            self.start = addr;
        }
    }

    fn trim_above(&mut self, addr: PAddr) {
        if self.start < addr && self.end > addr {
            self.end = addr;
        }
    }
}

type RegionVec = FixedVec<'static, MemoryRegion>;

lazy_static! {
    static ref REGIONS: spin::RwLock<RegionVec> = {
        const REGIONS_SIZE: usize = 256;
        static mut REGIONS_MEM: [MemoryRegion; REGIONS_SIZE] =
            [MemoryRegion::new(PAddr::from_u64(0), PAddr::from_u64(0));
             REGIONS_SIZE];
        // Unsafe to take a mutable reference of a static.
        // We instantly store it behind a Rwlock, so this is safe
        unsafe {
            spin::RwLock::new(FixedVec::new(&mut REGIONS_MEM))
        }
    };
}

/// Initial Rust entry point.
#[no_mangle]
pub extern "C" fn arch_init(multiboot_addr: PAddr) -> ! {
    unsafe {
        serial::init();
    }
    debug!("Serial Initialized");
    debug!("Multiboot Structure loaded at {:#X}", multiboot_addr);
    let mb = unsafe {
                 Multiboot::new(multiboot_addr.as_u64(), early_paddr_to_slice)
             }
             .expect("Could not access a Multiboot structure");

    {
        let mut regions = REGIONS.write();
        discover_memory(&mb, &mut *regions);
    }
    let regions = REGIONS.read();
    debug!("Available Memory:");
    for region in regions.iter() {
        debug!("{:#17X} - {:#17X}", region.start, region.end);
    }

    let allocator = FirstFitAllocator::get();
    populate_allocator(&*regions, allocator);
    map_available_memory(&*regions, allocator);
    loop {}
}

/// Report kernel physical memory range (not including boot code/data)
fn kernel_memory_range() -> (PAddr, PAddr) {
    // kbegin and kend are defined as symbols in the linker script
    extern "C" {
        static kbegin: u8;
        static kend: u8;
    }

    const MASK: u64 = (1 << 12) - 1;
    let kbegin_addr = {
        let ptr: *const _ = &kbegin;
        ptr as u64 & !MASK
    };
    let kend_addr = {
        let ptr: *const _ = &kend;
        (ptr as u64 + MASK) & !MASK
    };
    (PAddr::from_u64(kbegin_addr), PAddr::from_u64(kend_addr))
}

/// Discover available memory from the Multiboot structure and populate
/// `regions`
fn discover_memory(mb: &Multiboot, regions: &mut RegionVec) {
    let (kbegin, kend) = kernel_memory_range();
    debug!("kbegin = {:#X}, kend = {:#X}", kbegin, kend);
    let mem_regions = mb.memory_regions()
                        .expect("Could not find Multiboot memory map");
    for region in mem_regions {
        let start = PAddr::from_u64(region.base_address());
        let end = PAddr::from_u64(region.base_address() + region.length());
        let mem_type = match region.memory_type() {
            MemoryType::RAM => "RAM",
            MemoryType::Unusable => "Unusable",
        };
        info!("{:#17X} - {:#17X}: {}", start, end, mem_type);
        if region.memory_type() == MemoryType::RAM {
            let mut reg = MemoryRegion::new(start, end);
            reg.trim_below(kend);
            reg.trim_above(kbegin);
            if reg.start != reg.end {
                if let Err(e) = regions.push(reg) {
                    warn!("Could not store usable region {:#?}: {:?}",
                          region,
                          e)
                }
            }
        }
    }
}

/// Populate the memory allocator with accessible frames
fn populate_allocator<Allocator: FrameAllocator>(regions: &RegionVec,
                                                 allocator: &Allocator) {
    const INITIAL_MAP: PAddr = PAddr::from_u64(1 << 30);
    let accessible_frames = regions.iter().filter_map(|reg| {
        let start_frame = Frame::up(reg.start);
        let end_frame = Frame::down(if reg.end < INITIAL_MAP {
            reg.end
        } else {
            INITIAL_MAP
        });
        let range = FrameRange::new(start_frame, end_frame);
        if start_frame.start_address() >= INITIAL_MAP || range.nframes() == 0 {
            None
        } else {
            Some(range)
        }
    });
    for range in accessible_frames {
        unsafe { allocator.free_range_manual(range) };
    }
}

fn map_available_memory<Allocator: FrameAllocator>(regions: &RegionVec,
                                                   allocator: &Allocator) {


}
