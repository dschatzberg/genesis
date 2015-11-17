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

pub unsafe fn out<T>(port: u16, val: T) {
        asm!("out $0, $1"
             : // no output
             : "{al}" (val), "{dx}" (port)
             : // no clobber
             : "volatile");
}

pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
        asm!("in $1, $0"
             : "={al}" (val)
             : "{dx}" (port)
             : // no clobber
             : "volatile");
    val
}
