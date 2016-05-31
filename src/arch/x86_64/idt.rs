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
use core::mem;
use x86::dtables::*;
use x86::irq::*;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct ExceptionFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

/// Rust entry for all interrupts
#[no_mangle]
pub extern "C" fn interrupt_handler(num: usize, ef: u64) -> ! {
    if num < EXCEPTIONS.len() {
        error!("Received Exception: {}", EXCEPTIONS[num]);
    } else {
        error!("Recieved interrupt {}", num);
    }
    loop {}
}

pub fn init() {
    let idt_ptr = {
        let idtp: *const _ = &*IDT;
        let idt_size = (mem::size_of::<IdtEntry>() * IDT.len()) as u16;
        DescriptorTablePointer {
            limit: idt_size,
            base: idtp as u64,
        }
    };
    unsafe {
        lidt(&idt_ptr);
    }
}
lazy_static! {
    static ref IDT: [IdtEntry; 256] = {
        extern "C" {
            static int0: u8;
            static int1: u8;
            static int2: u8;
            static int3: u8;
            static int4: u8;
            static int5: u8;
            static int6: u8;
            static int7: u8;
            static int8: u8;
            static int9: u8;
            static int10: u8;
            static int11: u8;
            static int12: u8;
            static int13: u8;
            static int14: u8;
            static int15: u8;
            static int16: u8;
            static int17: u8;
            static int18: u8;
            static int19: u8;
            static int20: u8;
            static int32: u8;
            static int33: u8;
            static int34: u8;
            static int35: u8;
            static int36: u8;
            static int37: u8;
            static int38: u8;
            static int39: u8;
            static int40: u8;
            static int41: u8;
            static int42: u8;
            static int43: u8;
            static int44: u8;
            static int45: u8;
            static int46: u8;
            static int47: u8;
            static int48: u8;
            static int49: u8;
            static int50: u8;
            static int51: u8;
            static int52: u8;
            static int53: u8;
            static int54: u8;
            static int55: u8;
            static int56: u8;
            static int57: u8;
            static int58: u8;
            static int59: u8;
            static int60: u8;
            static int61: u8;
            static int62: u8;
            static int63: u8;
            static int64: u8;
            static int65: u8;
            static int66: u8;
            static int67: u8;
            static int68: u8;
            static int69: u8;
            static int70: u8;
            static int71: u8;
            static int72: u8;
            static int73: u8;
            static int74: u8;
            static int75: u8;
            static int76: u8;
            static int77: u8;
            static int78: u8;
            static int79: u8;
            static int80: u8;
            static int81: u8;
            static int82: u8;
            static int83: u8;
            static int84: u8;
            static int85: u8;
            static int86: u8;
            static int87: u8;
            static int88: u8;
            static int89: u8;
            static int90: u8;
            static int91: u8;
            static int92: u8;
            static int93: u8;
            static int94: u8;
            static int95: u8;
            static int96: u8;
            static int97: u8;
            static int98: u8;
            static int99: u8;
            static int100: u8;
            static int101: u8;
            static int102: u8;
            static int103: u8;
            static int104: u8;
            static int105: u8;
            static int106: u8;
            static int107: u8;
            static int108: u8;
            static int109: u8;
            static int110: u8;
            static int111: u8;
            static int112: u8;
            static int113: u8;
            static int114: u8;
            static int115: u8;
            static int116: u8;
            static int117: u8;
            static int118: u8;
            static int119: u8;
            static int120: u8;
            static int121: u8;
            static int122: u8;
            static int123: u8;
            static int124: u8;
            static int125: u8;
            static int126: u8;
            static int127: u8;
            static int128: u8;
            static int129: u8;
            static int130: u8;
            static int131: u8;
            static int132: u8;
            static int133: u8;
            static int134: u8;
            static int135: u8;
            static int136: u8;
            static int137: u8;
            static int138: u8;
            static int139: u8;
            static int140: u8;
            static int141: u8;
            static int142: u8;
            static int143: u8;
            static int144: u8;
            static int145: u8;
            static int146: u8;
            static int147: u8;
            static int148: u8;
            static int149: u8;
            static int150: u8;
            static int151: u8;
            static int152: u8;
            static int153: u8;
            static int154: u8;
            static int155: u8;
            static int156: u8;
            static int157: u8;
            static int158: u8;
            static int159: u8;
            static int160: u8;
            static int161: u8;
            static int162: u8;
            static int163: u8;
            static int164: u8;
            static int165: u8;
            static int166: u8;
            static int167: u8;
            static int168: u8;
            static int169: u8;
            static int170: u8;
            static int171: u8;
            static int172: u8;
            static int173: u8;
            static int174: u8;
            static int175: u8;
            static int176: u8;
            static int177: u8;
            static int178: u8;
            static int179: u8;
            static int180: u8;
            static int181: u8;
            static int182: u8;
            static int183: u8;
            static int184: u8;
            static int185: u8;
            static int186: u8;
            static int187: u8;
            static int188: u8;
            static int189: u8;
            static int190: u8;
            static int191: u8;
            static int192: u8;
            static int193: u8;
            static int194: u8;
            static int195: u8;
            static int196: u8;
            static int197: u8;
            static int198: u8;
            static int199: u8;
            static int200: u8;
            static int201: u8;
            static int202: u8;
            static int203: u8;
            static int204: u8;
            static int205: u8;
            static int206: u8;
            static int207: u8;
            static int208: u8;
            static int209: u8;
            static int210: u8;
            static int211: u8;
            static int212: u8;
            static int213: u8;
            static int214: u8;
            static int215: u8;
            static int216: u8;
            static int217: u8;
            static int218: u8;
            static int219: u8;
            static int220: u8;
            static int221: u8;
            static int222: u8;
            static int223: u8;
            static int224: u8;
            static int225: u8;
            static int226: u8;
            static int227: u8;
            static int228: u8;
            static int229: u8;
            static int230: u8;
            static int231: u8;
            static int232: u8;
            static int233: u8;
            static int234: u8;
            static int235: u8;
            static int236: u8;
            static int237: u8;
            static int238: u8;
            static int239: u8;
            static int240: u8;
            static int241: u8;
            static int242: u8;
            static int243: u8;
            static int244: u8;
            static int245: u8;
            static int246: u8;
            static int247: u8;
            static int248: u8;
            static int249: u8;
            static int250: u8;
            static int251: u8;
            static int252: u8;
            static int253: u8;
            static int254: u8;
            static int255: u8;
        }
        [IdtEntry::interrupt_gate(8, &int0),
        IdtEntry::interrupt_gate(8, &int1),
        IdtEntry::interrupt_gate(8, &int2),
        IdtEntry::interrupt_gate(8, &int3),
        IdtEntry::interrupt_gate(8, &int4),
        IdtEntry::interrupt_gate(8, &int5),
        IdtEntry::interrupt_gate(8, &int6),
        IdtEntry::interrupt_gate(8, &int7),
        IdtEntry::interrupt_gate(8, &int8),
        IdtEntry::interrupt_gate(8, &int9),
        IdtEntry::interrupt_gate(8, &int10),
        IdtEntry::interrupt_gate(8, &int11),
        IdtEntry::interrupt_gate(8, &int12),
        IdtEntry::interrupt_gate(8, &int13),
        IdtEntry::interrupt_gate(8, &int14),
        IdtEntry::interrupt_gate(8, &int15),
        IdtEntry::interrupt_gate(8, &int16),
        IdtEntry::interrupt_gate(8, &int17),
        IdtEntry::interrupt_gate(8, &int18),
        IdtEntry::interrupt_gate(8, &int19),
        IdtEntry::interrupt_gate(8, &int20),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::missing(),
        IdtEntry::interrupt_gate(8, &int32),
        IdtEntry::interrupt_gate(8, &int33),
        IdtEntry::interrupt_gate(8, &int34),
        IdtEntry::interrupt_gate(8, &int35),
        IdtEntry::interrupt_gate(8, &int36),
        IdtEntry::interrupt_gate(8, &int37),
        IdtEntry::interrupt_gate(8, &int38),
        IdtEntry::interrupt_gate(8, &int39),
        IdtEntry::interrupt_gate(8, &int40),
        IdtEntry::interrupt_gate(8, &int41),
        IdtEntry::interrupt_gate(8, &int42),
        IdtEntry::interrupt_gate(8, &int43),
        IdtEntry::interrupt_gate(8, &int44),
        IdtEntry::interrupt_gate(8, &int45),
        IdtEntry::interrupt_gate(8, &int46),
        IdtEntry::interrupt_gate(8, &int47),
        IdtEntry::interrupt_gate(8, &int48),
        IdtEntry::interrupt_gate(8, &int49),
        IdtEntry::interrupt_gate(8, &int50),
        IdtEntry::interrupt_gate(8, &int51),
        IdtEntry::interrupt_gate(8, &int52),
        IdtEntry::interrupt_gate(8, &int53),
        IdtEntry::interrupt_gate(8, &int54),
        IdtEntry::interrupt_gate(8, &int55),
        IdtEntry::interrupt_gate(8, &int56),
        IdtEntry::interrupt_gate(8, &int57),
        IdtEntry::interrupt_gate(8, &int58),
        IdtEntry::interrupt_gate(8, &int59),
        IdtEntry::interrupt_gate(8, &int60),
        IdtEntry::interrupt_gate(8, &int61),
        IdtEntry::interrupt_gate(8, &int62),
        IdtEntry::interrupt_gate(8, &int63),
        IdtEntry::interrupt_gate(8, &int64),
        IdtEntry::interrupt_gate(8, &int65),
        IdtEntry::interrupt_gate(8, &int66),
        IdtEntry::interrupt_gate(8, &int67),
        IdtEntry::interrupt_gate(8, &int68),
        IdtEntry::interrupt_gate(8, &int69),
        IdtEntry::interrupt_gate(8, &int70),
        IdtEntry::interrupt_gate(8, &int71),
        IdtEntry::interrupt_gate(8, &int72),
        IdtEntry::interrupt_gate(8, &int73),
        IdtEntry::interrupt_gate(8, &int74),
        IdtEntry::interrupt_gate(8, &int75),
        IdtEntry::interrupt_gate(8, &int76),
        IdtEntry::interrupt_gate(8, &int77),
        IdtEntry::interrupt_gate(8, &int78),
        IdtEntry::interrupt_gate(8, &int79),
        IdtEntry::interrupt_gate(8, &int80),
        IdtEntry::interrupt_gate(8, &int81),
        IdtEntry::interrupt_gate(8, &int82),
        IdtEntry::interrupt_gate(8, &int83),
        IdtEntry::interrupt_gate(8, &int84),
        IdtEntry::interrupt_gate(8, &int85),
        IdtEntry::interrupt_gate(8, &int86),
        IdtEntry::interrupt_gate(8, &int87),
        IdtEntry::interrupt_gate(8, &int88),
        IdtEntry::interrupt_gate(8, &int89),
        IdtEntry::interrupt_gate(8, &int90),
        IdtEntry::interrupt_gate(8, &int91),
        IdtEntry::interrupt_gate(8, &int92),
        IdtEntry::interrupt_gate(8, &int93),
        IdtEntry::interrupt_gate(8, &int94),
        IdtEntry::interrupt_gate(8, &int95),
        IdtEntry::interrupt_gate(8, &int96),
        IdtEntry::interrupt_gate(8, &int97),
        IdtEntry::interrupt_gate(8, &int98),
        IdtEntry::interrupt_gate(8, &int99),
        IdtEntry::interrupt_gate(8, &int100),
        IdtEntry::interrupt_gate(8, &int101),
        IdtEntry::interrupt_gate(8, &int102),
        IdtEntry::interrupt_gate(8, &int103),
        IdtEntry::interrupt_gate(8, &int104),
        IdtEntry::interrupt_gate(8, &int105),
        IdtEntry::interrupt_gate(8, &int106),
        IdtEntry::interrupt_gate(8, &int107),
        IdtEntry::interrupt_gate(8, &int108),
        IdtEntry::interrupt_gate(8, &int109),
        IdtEntry::interrupt_gate(8, &int110),
        IdtEntry::interrupt_gate(8, &int111),
        IdtEntry::interrupt_gate(8, &int112),
        IdtEntry::interrupt_gate(8, &int113),
        IdtEntry::interrupt_gate(8, &int114),
        IdtEntry::interrupt_gate(8, &int115),
        IdtEntry::interrupt_gate(8, &int116),
        IdtEntry::interrupt_gate(8, &int117),
        IdtEntry::interrupt_gate(8, &int118),
        IdtEntry::interrupt_gate(8, &int119),
        IdtEntry::interrupt_gate(8, &int120),
        IdtEntry::interrupt_gate(8, &int121),
        IdtEntry::interrupt_gate(8, &int122),
        IdtEntry::interrupt_gate(8, &int123),
        IdtEntry::interrupt_gate(8, &int124),
        IdtEntry::interrupt_gate(8, &int125),
        IdtEntry::interrupt_gate(8, &int126),
        IdtEntry::interrupt_gate(8, &int127),
        IdtEntry::interrupt_gate(8, &int128),
        IdtEntry::interrupt_gate(8, &int129),
        IdtEntry::interrupt_gate(8, &int130),
        IdtEntry::interrupt_gate(8, &int131),
        IdtEntry::interrupt_gate(8, &int132),
        IdtEntry::interrupt_gate(8, &int133),
        IdtEntry::interrupt_gate(8, &int134),
        IdtEntry::interrupt_gate(8, &int135),
        IdtEntry::interrupt_gate(8, &int136),
        IdtEntry::interrupt_gate(8, &int137),
        IdtEntry::interrupt_gate(8, &int138),
        IdtEntry::interrupt_gate(8, &int139),
        IdtEntry::interrupt_gate(8, &int140),
        IdtEntry::interrupt_gate(8, &int141),
        IdtEntry::interrupt_gate(8, &int142),
        IdtEntry::interrupt_gate(8, &int143),
        IdtEntry::interrupt_gate(8, &int144),
        IdtEntry::interrupt_gate(8, &int145),
        IdtEntry::interrupt_gate(8, &int146),
        IdtEntry::interrupt_gate(8, &int147),
        IdtEntry::interrupt_gate(8, &int148),
        IdtEntry::interrupt_gate(8, &int149),
        IdtEntry::interrupt_gate(8, &int150),
        IdtEntry::interrupt_gate(8, &int151),
        IdtEntry::interrupt_gate(8, &int152),
        IdtEntry::interrupt_gate(8, &int153),
        IdtEntry::interrupt_gate(8, &int154),
        IdtEntry::interrupt_gate(8, &int155),
        IdtEntry::interrupt_gate(8, &int156),
        IdtEntry::interrupt_gate(8, &int157),
        IdtEntry::interrupt_gate(8, &int158),
        IdtEntry::interrupt_gate(8, &int159),
        IdtEntry::interrupt_gate(8, &int160),
        IdtEntry::interrupt_gate(8, &int161),
        IdtEntry::interrupt_gate(8, &int162),
        IdtEntry::interrupt_gate(8, &int163),
        IdtEntry::interrupt_gate(8, &int164),
        IdtEntry::interrupt_gate(8, &int165),
        IdtEntry::interrupt_gate(8, &int166),
        IdtEntry::interrupt_gate(8, &int167),
        IdtEntry::interrupt_gate(8, &int168),
        IdtEntry::interrupt_gate(8, &int169),
        IdtEntry::interrupt_gate(8, &int170),
        IdtEntry::interrupt_gate(8, &int171),
        IdtEntry::interrupt_gate(8, &int172),
        IdtEntry::interrupt_gate(8, &int173),
        IdtEntry::interrupt_gate(8, &int174),
        IdtEntry::interrupt_gate(8, &int175),
        IdtEntry::interrupt_gate(8, &int176),
        IdtEntry::interrupt_gate(8, &int177),
        IdtEntry::interrupt_gate(8, &int178),
        IdtEntry::interrupt_gate(8, &int179),
        IdtEntry::interrupt_gate(8, &int180),
        IdtEntry::interrupt_gate(8, &int181),
        IdtEntry::interrupt_gate(8, &int182),
        IdtEntry::interrupt_gate(8, &int183),
        IdtEntry::interrupt_gate(8, &int184),
        IdtEntry::interrupt_gate(8, &int185),
        IdtEntry::interrupt_gate(8, &int186),
        IdtEntry::interrupt_gate(8, &int187),
        IdtEntry::interrupt_gate(8, &int188),
        IdtEntry::interrupt_gate(8, &int189),
        IdtEntry::interrupt_gate(8, &int190),
        IdtEntry::interrupt_gate(8, &int191),
        IdtEntry::interrupt_gate(8, &int192),
        IdtEntry::interrupt_gate(8, &int193),
        IdtEntry::interrupt_gate(8, &int194),
        IdtEntry::interrupt_gate(8, &int195),
        IdtEntry::interrupt_gate(8, &int196),
        IdtEntry::interrupt_gate(8, &int197),
        IdtEntry::interrupt_gate(8, &int198),
        IdtEntry::interrupt_gate(8, &int199),
        IdtEntry::interrupt_gate(8, &int200),
        IdtEntry::interrupt_gate(8, &int201),
        IdtEntry::interrupt_gate(8, &int202),
        IdtEntry::interrupt_gate(8, &int203),
        IdtEntry::interrupt_gate(8, &int204),
        IdtEntry::interrupt_gate(8, &int205),
        IdtEntry::interrupt_gate(8, &int206),
        IdtEntry::interrupt_gate(8, &int207),
        IdtEntry::interrupt_gate(8, &int208),
        IdtEntry::interrupt_gate(8, &int209),
        IdtEntry::interrupt_gate(8, &int210),
        IdtEntry::interrupt_gate(8, &int211),
        IdtEntry::interrupt_gate(8, &int212),
        IdtEntry::interrupt_gate(8, &int213),
        IdtEntry::interrupt_gate(8, &int214),
        IdtEntry::interrupt_gate(8, &int215),
        IdtEntry::interrupt_gate(8, &int216),
        IdtEntry::interrupt_gate(8, &int217),
        IdtEntry::interrupt_gate(8, &int218),
        IdtEntry::interrupt_gate(8, &int219),
        IdtEntry::interrupt_gate(8, &int220),
        IdtEntry::interrupt_gate(8, &int221),
        IdtEntry::interrupt_gate(8, &int222),
        IdtEntry::interrupt_gate(8, &int223),
        IdtEntry::interrupt_gate(8, &int224),
        IdtEntry::interrupt_gate(8, &int225),
        IdtEntry::interrupt_gate(8, &int226),
        IdtEntry::interrupt_gate(8, &int227),
        IdtEntry::interrupt_gate(8, &int228),
        IdtEntry::interrupt_gate(8, &int229),
        IdtEntry::interrupt_gate(8, &int230),
        IdtEntry::interrupt_gate(8, &int231),
        IdtEntry::interrupt_gate(8, &int232),
        IdtEntry::interrupt_gate(8, &int233),
        IdtEntry::interrupt_gate(8, &int234),
        IdtEntry::interrupt_gate(8, &int235),
        IdtEntry::interrupt_gate(8, &int236),
        IdtEntry::interrupt_gate(8, &int237),
        IdtEntry::interrupt_gate(8, &int238),
        IdtEntry::interrupt_gate(8, &int239),
        IdtEntry::interrupt_gate(8, &int240),
        IdtEntry::interrupt_gate(8, &int241),
        IdtEntry::interrupt_gate(8, &int242),
        IdtEntry::interrupt_gate(8, &int243),
        IdtEntry::interrupt_gate(8, &int244),
        IdtEntry::interrupt_gate(8, &int245),
        IdtEntry::interrupt_gate(8, &int246),
        IdtEntry::interrupt_gate(8, &int247),
        IdtEntry::interrupt_gate(8, &int248),
        IdtEntry::interrupt_gate(8, &int249),
        IdtEntry::interrupt_gate(8, &int250),
        IdtEntry::interrupt_gate(8, &int251),
        IdtEntry::interrupt_gate(8, &int252),
        IdtEntry::interrupt_gate(8, &int253),
        IdtEntry::interrupt_gate(8, &int254),
        IdtEntry::interrupt_gate(8, &int255)]
    };
}
