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
use core::mem::{size_of, transmute};
use core::slice;
use fixedvec::FixedVec;
use mem::PAddr;
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

#[derive(Clone, Copy, Debug, Default)]
struct MemoryRegion {
    start: PAddr,
    end: PAddr,
}

impl MemoryRegion {
    fn new(start: PAddr, end: PAddr) -> MemoryRegion {
        MemoryRegion {
            start: start,
            end: end,
        }
    }

    fn trim_below(&mut self, addr: PAddr) -> () {
        if self.start < addr && self.end > addr {
            self.start = addr;
        }
    }

    fn trim_above(&mut self, addr: PAddr) -> () {
        if self.start < addr && self.end > addr {
            self.end = addr;
        }
    }
}

lazy_static! {
    static ref REGIONS: spin::RwLock<FixedVec<'static, MemoryRegion>> = {
        const REGIONS_SIZE: usize = 4096;
        static mut REGIONS_MEM: [u8; REGIONS_SIZE] = [0; REGIONS_SIZE];
        let region_slice = unsafe {
            let ptr = transmute(REGIONS_MEM.as_mut_ptr());
            let sz = REGIONS_SIZE / size_of::<MemoryRegion>();
            slice::from_raw_parts_mut(ptr, sz)
        };

        spin::RwLock::new(FixedVec::new(region_slice))
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

    discover_memory(&mb);

    loop {}
}

/// Report kernel physical memory range (not including boot code/data)
fn kernel_memory_range() -> (PAddr, PAddr) {
    extern {
        static kbegin: u8;
        static kend: u8;
    }

    let mask = (1 << 12) - 1;
    let kbegin_addr = {
        let ptr: *const _ = &kbegin;
        ptr as u64 & !mask
    };
    let kend_addr = {
        let ptr: *const _ = &kend;
        (ptr as u64 + mask) & !mask
    };
    (PAddr::from_u64(kbegin_addr), PAddr::from_u64(kend_addr))
}

/// Discover available memory from the Multiboot structure
fn discover_memory(mb: &Multiboot) -> () {
    let mem_regions = mb.memory_regions()
                        .expect("Could not find Multiboot memory map");

    let mut vec = REGIONS.write();

    let (kbegin, kend) = kernel_memory_range();
    debug!("kbegin = {:#X}, kend = {:#X}", kbegin, kend);
    info!("Memory Map:");
    for region in mem_regions {
        let start = PAddr::from_u64(region.base_address());
        let end = PAddr::from_u64(region.base_address() + region.length());
        let mem_type = match region.memory_type() {
            MemoryType::RAM => "RAM",
            MemoryType::Unusable => "Unusable",
        };
        info!("{:#17X} - {:#17X}: {}", start, end, mem_type);
        if region.memory_type() == MemoryType::RAM {
            let mut region = MemoryRegion::new(start, end);
            region.trim_below(kend);
            region.trim_above(kbegin);
            if let Err(e) = vec.push(region) {
                warn!("Could not store usable region {:#?}: {:?}", region, e);
            }
        }
    }

    debug!("Available Memory:");
    for region in vec.iter() {
        debug!("{:?}", region);
    }
}
