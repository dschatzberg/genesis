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
use mem::PAddr;
use multiboot::{self, Multiboot};

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

/// Initial Rust entry point.
#[no_mangle]
pub extern "C" fn arch_init(multiboot_addr: PAddr) -> ! {
    unsafe {
        serial::init();
    }
    debug!("Serial Initialized");
    info!("Multiboot Structure loaded at {:#X}", multiboot_addr);
    let mb = unsafe {
                 Multiboot::new(multiboot_addr.as_u64(), early_paddr_to_slice)
             }
             .expect("Could not access a Multiboot structure");
    let mem_regions = mb.memory_regions()
                        .expect("Could not find Multiboot memory map");
    for region in mem_regions {
        info!("Multiboot Region {:#?}", region);
    }

    loop {}
}
