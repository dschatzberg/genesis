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
use std::mem;
use memory::*;
use memory::first_fit_allocator::FirstFitAllocator;
use multiboot::{self, MemoryType, Multiboot};
use spin;
use std::u32;
use x86::segmentation::*;
use x86::dtables::*;

/// Initial Rust entry point.
#[no_mangle]
pub extern "C" fn arch_init(multiboot_addr: PAddr) -> ! {
    initialize_console();

    process_multiboot(multiboot_addr);

    let regions = REGIONS.read();
    let allocator = FirstFitAllocator::get();
    populate_allocator(&*regions, allocator);

    let (page_table_frame, mut page_table) =
        create_runtime_pagetable(allocator);

    map_free_memory(&mut page_table, &*regions, allocator);
    map_kernel(&mut page_table, allocator);
    let new_stack = map_stack(&mut page_table, allocator).as_usize();

    // Drop the REGIONS read lock before switching the page table
    drop(regions);

    unsafe {
        switch_to_runtime_pagetable(new_stack as u64,
                                    page_table_frame.start_address().as_u64(),
                                    arch_continue_init);
    }
}

extern "C" fn arch_continue_init() -> ! {
    let regions = REGIONS.read();
    let allocator = FirstFitAllocator::get();
    // Now that we are on the runtime page table, we can free boot and higher
    // memory to the allocator
    free_boot_memory(allocator);
    free_upper_memory(&regions, allocator);

    reset_gdt();
    loop {}
}

fn initialize_console() {
    unsafe {
        serial::init();
    }
    debug!("Serial Initialized");
}

fn process_multiboot(multiboot_addr: PAddr) {
    debug!("Multiboot Structure loaded at {:#X}", multiboot_addr);
    let mb = unsafe {
                 Multiboot::new(multiboot_addr.as_u64(), early_paddr_to_slice)
             }
             .expect("Could not access a Multiboot structure");

    process_multiboot_memory(mb.memory_regions()
                               .expect("Could not find Multiboot memory map"));

    // TODO: process cmdline and modules
}

/// Discover available memory from the Multiboot structure and populate
/// `REGIONS`
fn process_multiboot_memory(mem_regions: multiboot::MemoryMapIter) {
    let mut regions = REGIONS.write();
    // kbegin and kend are defined as symbols in the linker script
    let (kbegin, kend) = {
        extern "C" {
            static kend: u8;
            static kbegin: u8;
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
    };
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
            if reg.start < reg.end {
                if let Err(e) = regions.push(reg) {
                    warn!("Could not store usable region {:#?}: {:?}",
                          region,
                          e)
                }
            }
        }
    }
}

const INITIAL_VIRTUAL_OFFSET: u64 = 0xFFFFFFFFC0000000;

unsafe fn early_paddr_to_slice<'a>(p: multiboot::PAddr,
                                   sz: usize)
                                   -> Option<&'a [u8]> {
    if (sz < (1 << 30)) && ((p + sz as u64) < (1 << 30)) {
        let ptr = transmute(p + INITIAL_VIRTUAL_OFFSET);
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

    static ref GDT: [SegmentDescriptor; 5] =
        [SegmentDescriptor::empty(), // NULL Descriptor
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_C_ER | DESC_S | DESC_DPL0 | DESC_P | DESC_L | DESC_G,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_D_RW | DESC_S | DESC_DPL0 | DESC_P,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_D_RW | DESC_S | DESC_DPL3 | DESC_P,
         SegmentDescriptor::new(0, u32::MAX) |
         TYPE_C_ER | DESC_S | DESC_DPL3 | DESC_P | DESC_L | DESC_G];
}

/// Reports text segment, read only data, and writable data
fn kernel_segments() -> (FrameRange, FrameRange, FrameRange) {
    let convert = |sym: &'static u8| {
        let addr: *const _ = sym;
        PAddr::from_u64(addr as u64 - INITIAL_VIRTUAL_OFFSET)
    };
    let range = |sym1: &'static u8, sym2: &'static u8| {
        FrameRange::new(Frame::down(convert(sym1)), Frame::up(convert(sym2)))
    };
    let text_range = {
        extern "C" {
            static btext: u8;
            static etext: u8;
        }
        range(&btext, &etext)
    };
    let ro_range = {
        extern "C" {
            static bro: u8;
            static ero: u8;
        }
        range(&bro, &ero)
    };
    let data_range = {
        extern "C" {
            static bdata: u8;
            static edata: u8;
        }
        range(&bdata, &edata)
    };
    (text_range, ro_range, data_range)
}

const INITIAL_MAP: PAddr = PAddr::from_u64(1 << 30);

/// Populate the memory allocator with accessible frames
fn populate_allocator<Allocator: FrameAllocator>(regions: &RegionVec,
                                                 allocator: &Allocator) {
    let boot_begin = {
        extern "C" {
            static boot_begin: u8;
        }
        let ptr: *const _ = &boot_begin;
        PAddr::from_u64(ptr as u64)
    };
    let accessible_frames = regions.iter().filter_map(|reg| {
        let mut region = *reg;
        region.trim_above(INITIAL_MAP);
        region.trim_above(boot_begin);
        let start_frame = Frame::up(region.start);
        let end_frame = Frame::down(region.end);
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

fn create_runtime_pagetable<Allocator: FrameAllocator>
    (allocator: &Allocator)
     -> (Frame, PageTable) {
    let frame = allocator.allocate_manual()
                         .expect("Could not allocate frame for new PageTable");
    for b in initial_frame_to_slice(frame).iter_mut() {
        *b = 0;
    }
    unsafe {
        let table = (frame.start_address().as_u64() +
                     0xFFFFFFFFC0000000) as *mut PML4;
        (frame, PageTable::new(table))
    }
}

type PageSlice = [u8; PAGE_SIZE as usize];

fn initial_frame_to_slice<'a>(frame: Frame) -> &'a mut PageSlice {
    unsafe {
        transmute(frame.start_address().as_u64() + INITIAL_VIRTUAL_OFFSET)
    }
}

fn map_free_memory<Allocator: FrameAllocator>(page_table: &mut PageTable,
                                              regions: &RegionVec,
                                              allocator: &Allocator) {
    let frames = regions.iter().filter_map(|reg| {
        let start_frame = Frame::up(reg.start);
        let end_frame = Frame::down(reg.end);
        let range = FrameRange::new(start_frame, end_frame);
        if range.nframes() == 0 {
            None
        } else {
            Some(range)
        }
    });

    for range in frames {
        for offset in 0..range.nframes() {
            let frame = range.lower() + offset;
            let page = {
                assert!(frame.start_address() < INITIAL_MAP);
                let addr = frame.start_address().as_u64() +
                           0xFFFF_FF80_0000_0000;
                Page::down(VAddr::from_usize(addr as usize))
            };
            map(page_table,
                page,
                frame,
                PT_P | PT_RW | PT_G | PT_XD,
                allocator,
                initial_frame_to_slice);
        }
    }
}

fn map_kernel<Allocator>(page_table: &mut PageTable, allocator: &Allocator)
    where Allocator: FrameAllocator
{
    let (text_range, ro_range, data_range) = kernel_segments();
    map_range(page_table, text_range, PT_P | PT_G, allocator);
    map_range(page_table, ro_range, PT_P | PT_G | PT_XD, allocator);
    map_range(page_table,
              data_range,
              PT_P | PT_RW | PT_G | PT_XD,
              allocator);
}

fn map_range<Allocator>(page_table: &mut PageTable,
                        range: FrameRange,
                        flags: PTEntry,
                        allocator: &Allocator)
    where Allocator: FrameAllocator
{
    for offset in 0..range.nframes() {
        let frame = range.lower() + offset;
        let page = {
            assert!(frame.start_address() < INITIAL_MAP);
            let addr = frame.start_address().as_u64() + INITIAL_VIRTUAL_OFFSET;
            Page::down(VAddr::from_usize(addr as usize))
        };
        map(page_table,
            page,
            frame,
            flags,
            allocator,
            initial_frame_to_slice);
    }
}

// map a 12K stack with a guard page below the kernel start
fn map_stack<Allocator>(page_table: &mut PageTable,
                        allocator: &Allocator)
                        -> VAddr
    where Allocator: FrameAllocator
{
    let kbegin_page = {
        extern "C" {
            static kbegin: u8;
        }
        let ptr: *const _ = &kbegin;
        Page::down(VAddr::from_usize(ptr as usize))
    };
    for i in 1..3 {
        let frame = allocator.allocate_manual()
                             .expect("Could not allocate frame for stack");
        let page = kbegin_page - i;
        map(page_table,
            page,
            frame,
            PT_P | PT_RW | PT_G | PT_XD,
            allocator,
            initial_frame_to_slice);
    }
    kbegin_page.start_address()
}

extern "C" {
    fn switch_to_runtime_pagetable(stack: u64,
                                   pml4: u64,
                                   cb: extern "C" fn() -> !)
                                   -> !;
}

fn free_boot_memory<Allocator>(allocator: &Allocator)
    where Allocator: FrameAllocator
{
    let boot_begin = {
        extern "C" {
            static boot_begin: u8;
        }
        let ptr: *const _ = &boot_begin;
        PAddr::from_u64(ptr as u64)
    };
    let kbegin = {
        extern "C" {
            static kbegin: u8;
        }
        let ptr: *const _ = &kbegin;
        PAddr::from_u64(ptr as u64)
    };
    let start_frame = Frame::up(boot_begin);
    let end_frame = Frame::down(kbegin);
    let range = FrameRange::new(start_frame, end_frame);
    unsafe {
        allocator.free_range_manual(range);
    }
}

fn free_upper_memory<Allocator>(regions: &RegionVec, allocator: &Allocator)
    where Allocator: FrameAllocator
{
    let accessible_frames = regions.iter().filter_map(|reg| {
        let mut region = *reg;
        region.trim_below(INITIAL_MAP);
        let start_frame = Frame::up(region.start);
        let end_frame = Frame::down(region.end);
        let range = FrameRange::new(start_frame, end_frame);
        if end_frame.start_address() <= INITIAL_MAP || range.nframes() == 0 {
            None
        } else {
            Some(range)
        }
    });
    for range in accessible_frames {
        unsafe { allocator.free_range_manual(range) };
    }
}

fn reset_gdt() {
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

fn map<'a, Allocator, F>(page_table: &'a mut PageTable,
                         page: Page,
                         frame: Frame,
                         flags: PTEntry,
                         allocator: &Allocator,
                         f: F)
    where Allocator: FrameAllocator,
          F: Fn(Frame) -> &'a mut [u8; PAGE_SIZE as usize]
{
    let pml4 = page_table.get_mut();
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
        transmute(f(Frame::down(pml4[pml4_idx].get_address())))
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
        transmute(f(Frame::down(pdpt[pdpt_idx].get_address())))
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
        transmute(f(Frame::down(pd[pd_idx].get_address())))
    };
    let pt_idx = pt_index(page.start_address());
    assert!(pt[pt_idx].is_empty());
    pt[pt_idx] = PTEntry::new(frame.start_address(), flags);
}
