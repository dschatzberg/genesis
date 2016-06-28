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
#![allow(trivial_casts)]

use core::mem;
use x86::dtables::*;
use x86::irq::*;
use super::mem::VAddr;

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
    assert_has_not_been_called!("idt::init() function \
                                 must only be called once");
    populate_idt();
    let idt_ptr = {
        let idtp: *const _ = unsafe { &IDT };
        let idt_len = unsafe { IDT.len() };
        let idt_size = (mem::size_of::<IdtEntry>() * idt_len) as u16;
        DescriptorTablePointer {
            limit: idt_size,
            base: idtp as u64,
        }
    };
    unsafe {
        lidt(&idt_ptr);
    }
}

fn populate_idt() {
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
    let idt = unsafe { &mut IDT };
    populate_entry(idt, 0, &int0);
    populate_entry(idt, 1, &int1);
    populate_entry(idt, 2, &int2);
    populate_entry(idt, 3, &int3);
    populate_entry(idt, 4, &int4);
    populate_entry(idt, 5, &int5);
    populate_entry(idt, 6, &int6);
    populate_entry(idt, 7, &int7);
    populate_entry(idt, 8, &int8);
    populate_entry(idt, 9, &int9);
    populate_entry(idt, 10, &int10);
    populate_entry(idt, 11, &int11);
    populate_entry(idt, 12, &int12);
    populate_entry(idt, 13, &int13);
    populate_entry(idt, 14, &int14);
    populate_entry(idt, 15, &int15);
    populate_entry(idt, 16, &int16);
    populate_entry(idt, 17, &int17);
    populate_entry(idt, 18, &int18);
    populate_entry(idt, 19, &int19);
    populate_entry(idt, 20, &int20);

    populate_entry(idt, 32, &int32);
    populate_entry(idt, 33, &int33);
    populate_entry(idt, 34, &int34);
    populate_entry(idt, 35, &int35);
    populate_entry(idt, 36, &int36);
    populate_entry(idt, 37, &int37);
    populate_entry(idt, 38, &int38);
    populate_entry(idt, 39, &int39);
    populate_entry(idt, 40, &int40);
    populate_entry(idt, 41, &int41);
    populate_entry(idt, 42, &int42);
    populate_entry(idt, 43, &int43);
    populate_entry(idt, 44, &int44);
    populate_entry(idt, 45, &int45);
    populate_entry(idt, 46, &int46);
    populate_entry(idt, 47, &int47);
    populate_entry(idt, 48, &int48);
    populate_entry(idt, 49, &int49);
    populate_entry(idt, 50, &int50);
    populate_entry(idt, 51, &int51);
    populate_entry(idt, 52, &int52);
    populate_entry(idt, 53, &int53);
    populate_entry(idt, 54, &int54);
    populate_entry(idt, 55, &int55);
    populate_entry(idt, 56, &int56);
    populate_entry(idt, 57, &int57);
    populate_entry(idt, 58, &int58);
    populate_entry(idt, 59, &int59);
    populate_entry(idt, 60, &int60);
    populate_entry(idt, 61, &int61);
    populate_entry(idt, 62, &int62);
    populate_entry(idt, 63, &int63);
    populate_entry(idt, 64, &int64);
    populate_entry(idt, 65, &int65);
    populate_entry(idt, 66, &int66);
    populate_entry(idt, 67, &int67);
    populate_entry(idt, 68, &int68);
    populate_entry(idt, 69, &int69);
    populate_entry(idt, 70, &int70);
    populate_entry(idt, 71, &int71);
    populate_entry(idt, 72, &int72);
    populate_entry(idt, 73, &int73);
    populate_entry(idt, 74, &int74);
    populate_entry(idt, 75, &int75);
    populate_entry(idt, 76, &int76);
    populate_entry(idt, 77, &int77);
    populate_entry(idt, 78, &int78);
    populate_entry(idt, 79, &int79);
    populate_entry(idt, 80, &int80);
    populate_entry(idt, 81, &int81);
    populate_entry(idt, 82, &int82);
    populate_entry(idt, 83, &int83);
    populate_entry(idt, 84, &int84);
    populate_entry(idt, 85, &int85);
    populate_entry(idt, 86, &int86);
    populate_entry(idt, 87, &int87);
    populate_entry(idt, 88, &int88);
    populate_entry(idt, 89, &int89);
    populate_entry(idt, 90, &int90);
    populate_entry(idt, 91, &int91);
    populate_entry(idt, 92, &int92);
    populate_entry(idt, 93, &int93);
    populate_entry(idt, 94, &int94);
    populate_entry(idt, 95, &int95);
    populate_entry(idt, 96, &int96);
    populate_entry(idt, 97, &int97);
    populate_entry(idt, 98, &int98);
    populate_entry(idt, 99, &int99);
    populate_entry(idt, 100, &int100);
    populate_entry(idt, 101, &int101);
    populate_entry(idt, 102, &int102);
    populate_entry(idt, 103, &int103);
    populate_entry(idt, 104, &int104);
    populate_entry(idt, 105, &int105);
    populate_entry(idt, 106, &int106);
    populate_entry(idt, 107, &int107);
    populate_entry(idt, 108, &int108);
    populate_entry(idt, 109, &int109);
    populate_entry(idt, 110, &int110);
    populate_entry(idt, 111, &int111);
    populate_entry(idt, 112, &int112);
    populate_entry(idt, 113, &int113);
    populate_entry(idt, 114, &int114);
    populate_entry(idt, 115, &int115);
    populate_entry(idt, 116, &int116);
    populate_entry(idt, 117, &int117);
    populate_entry(idt, 118, &int118);
    populate_entry(idt, 119, &int119);
    populate_entry(idt, 120, &int120);
    populate_entry(idt, 121, &int121);
    populate_entry(idt, 122, &int122);
    populate_entry(idt, 123, &int123);
    populate_entry(idt, 124, &int124);
    populate_entry(idt, 125, &int125);
    populate_entry(idt, 126, &int126);
    populate_entry(idt, 127, &int127);
    populate_entry(idt, 128, &int128);
    populate_entry(idt, 129, &int129);
    populate_entry(idt, 130, &int130);
    populate_entry(idt, 131, &int131);
    populate_entry(idt, 132, &int132);
    populate_entry(idt, 133, &int133);
    populate_entry(idt, 134, &int134);
    populate_entry(idt, 135, &int135);
    populate_entry(idt, 136, &int136);
    populate_entry(idt, 137, &int137);
    populate_entry(idt, 138, &int138);
    populate_entry(idt, 139, &int139);
    populate_entry(idt, 140, &int140);
    populate_entry(idt, 141, &int141);
    populate_entry(idt, 142, &int142);
    populate_entry(idt, 143, &int143);
    populate_entry(idt, 144, &int144);
    populate_entry(idt, 145, &int145);
    populate_entry(idt, 146, &int146);
    populate_entry(idt, 147, &int147);
    populate_entry(idt, 148, &int148);
    populate_entry(idt, 149, &int149);
    populate_entry(idt, 150, &int150);
    populate_entry(idt, 151, &int151);
    populate_entry(idt, 152, &int152);
    populate_entry(idt, 153, &int153);
    populate_entry(idt, 154, &int154);
    populate_entry(idt, 155, &int155);
    populate_entry(idt, 156, &int156);
    populate_entry(idt, 157, &int157);
    populate_entry(idt, 158, &int158);
    populate_entry(idt, 159, &int159);
    populate_entry(idt, 160, &int160);
    populate_entry(idt, 161, &int161);
    populate_entry(idt, 162, &int162);
    populate_entry(idt, 163, &int163);
    populate_entry(idt, 164, &int164);
    populate_entry(idt, 165, &int165);
    populate_entry(idt, 166, &int166);
    populate_entry(idt, 167, &int167);
    populate_entry(idt, 168, &int168);
    populate_entry(idt, 169, &int169);
    populate_entry(idt, 170, &int170);
    populate_entry(idt, 171, &int171);
    populate_entry(idt, 172, &int172);
    populate_entry(idt, 173, &int173);
    populate_entry(idt, 174, &int174);
    populate_entry(idt, 175, &int175);
    populate_entry(idt, 176, &int176);
    populate_entry(idt, 177, &int177);
    populate_entry(idt, 178, &int178);
    populate_entry(idt, 179, &int179);
    populate_entry(idt, 180, &int180);
    populate_entry(idt, 181, &int181);
    populate_entry(idt, 182, &int182);
    populate_entry(idt, 183, &int183);
    populate_entry(idt, 184, &int184);
    populate_entry(idt, 185, &int185);
    populate_entry(idt, 186, &int186);
    populate_entry(idt, 187, &int187);
    populate_entry(idt, 188, &int188);
    populate_entry(idt, 189, &int189);
    populate_entry(idt, 190, &int190);
    populate_entry(idt, 191, &int191);
    populate_entry(idt, 192, &int192);
    populate_entry(idt, 193, &int193);
    populate_entry(idt, 194, &int194);
    populate_entry(idt, 195, &int195);
    populate_entry(idt, 196, &int196);
    populate_entry(idt, 197, &int197);
    populate_entry(idt, 198, &int198);
    populate_entry(idt, 199, &int199);
    populate_entry(idt, 200, &int200);
    populate_entry(idt, 201, &int201);
    populate_entry(idt, 202, &int202);
    populate_entry(idt, 203, &int203);
    populate_entry(idt, 204, &int204);
    populate_entry(idt, 205, &int205);
    populate_entry(idt, 206, &int206);
    populate_entry(idt, 207, &int207);
    populate_entry(idt, 208, &int208);
    populate_entry(idt, 209, &int209);
    populate_entry(idt, 210, &int210);
    populate_entry(idt, 211, &int211);
    populate_entry(idt, 212, &int212);
    populate_entry(idt, 213, &int213);
    populate_entry(idt, 214, &int214);
    populate_entry(idt, 215, &int215);
    populate_entry(idt, 216, &int216);
    populate_entry(idt, 217, &int217);
    populate_entry(idt, 218, &int218);
    populate_entry(idt, 219, &int219);
    populate_entry(idt, 220, &int220);
    populate_entry(idt, 221, &int221);
    populate_entry(idt, 222, &int222);
    populate_entry(idt, 223, &int223);
    populate_entry(idt, 224, &int224);
    populate_entry(idt, 225, &int225);
    populate_entry(idt, 226, &int226);
    populate_entry(idt, 227, &int227);
    populate_entry(idt, 228, &int228);
    populate_entry(idt, 229, &int229);
    populate_entry(idt, 230, &int230);
    populate_entry(idt, 231, &int231);
    populate_entry(idt, 232, &int232);
    populate_entry(idt, 233, &int233);
    populate_entry(idt, 234, &int234);
    populate_entry(idt, 235, &int235);
    populate_entry(idt, 236, &int236);
    populate_entry(idt, 237, &int237);
    populate_entry(idt, 238, &int238);
    populate_entry(idt, 239, &int239);
    populate_entry(idt, 240, &int240);
    populate_entry(idt, 241, &int241);
    populate_entry(idt, 242, &int242);
    populate_entry(idt, 243, &int243);
    populate_entry(idt, 244, &int244);
    populate_entry(idt, 245, &int245);
    populate_entry(idt, 246, &int246);
    populate_entry(idt, 247, &int247);
    populate_entry(idt, 248, &int248);
    populate_entry(idt, 249, &int249);
    populate_entry(idt, 250, &int250);
    populate_entry(idt, 251, &int251);
    populate_entry(idt, 252, &int252);
    populate_entry(idt, 253, &int253);
    populate_entry(idt, 254, &int254);
    populate_entry(idt, 255, &int255);
}

fn populate_entry(idt: &mut [IdtEntry; 256], ind: usize, addr: &'static u8) {
    let vaddr = VAddr::from_usize(addr as *const _ as usize);
    idt[ind] = IdtEntry::interrupt_gate(8, vaddr);
}
static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];
