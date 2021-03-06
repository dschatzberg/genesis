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
#![allow(dead_code)]

use super::ioport;

const PORT_BASE: u16 = 0x3F8;
// when DLAB = 0
const DATA_REG: u16 = 0;
const INT_ENABLE: u16 = 1;

// when DLAB = 1
const BAUD_DIV_LSB: u16 = 0;
const BAUD_DIV_MSB: u16 = 1;

const LINE_CTRL_REG: u16 = 3;
const LINE_CTRL_REG_CHARLEN8: u8 = 1 | 1 << 1;
const LINE_CTRL_REG_DLAB: u8 = 1 << 7;

const LINE_STATUS_REG: u16 = 5;
const LINE_STATUS_REG_THR_EMPTY: u8 = 1 << 5;

/// Initialize the Serial Port
pub fn init() {
    assert_has_not_been_called!("serial::init() function \
                                 must only be called once");
    unsafe {
        ioport::out(PORT_BASE + INT_ENABLE, 0u8); // disable interrupts

        // enable dlab
        ioport::out(PORT_BASE + LINE_CTRL_REG, LINE_CTRL_REG_DLAB);
        // XXX: hard coded 115200 baud
        ioport::out(PORT_BASE + BAUD_DIV_LSB, 1u8);
        ioport::out(PORT_BASE + BAUD_DIV_MSB, 0u8);

        // XXX: hard coded as 8N1 (8 bits, no parity, one stop bit)
        ioport::out(PORT_BASE + LINE_CTRL_REG, LINE_CTRL_REG_CHARLEN8);
    }
}

unsafe fn is_transmit_empty() -> bool {
    ioport::inb(PORT_BASE + LINE_STATUS_REG) & LINE_STATUS_REG_THR_EMPTY != 0
}

unsafe fn putc(c: u8) {
    while !is_transmit_empty() {}

    ioport::out(PORT_BASE + DATA_REG, c);
}

/// Write `str` to the Serial Port
pub unsafe fn write_str(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}
