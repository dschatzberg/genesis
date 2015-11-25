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
use core::cmp::Ordering;
use core::fmt;

#[derive(Copy, Clone, Debug, Default)]
pub struct PAddr(u64);

#[derive(Copy, Clone, Debug, Default)]
pub struct VAddr(usize);

impl PAddr {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    pub fn from_u64(v: u64) -> Self {
        PAddr(v)
    }
}

impl fmt::Binary for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Eq for PAddr {}
impl PartialEq for PAddr {
    fn eq(&self, other: &PAddr) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for PAddr {
    fn partial_cmp(&self, other: &PAddr) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for PAddr {
    fn cmp(&self, other: &PAddr) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Binary for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Eq for VAddr {}
impl PartialEq for VAddr {
    fn eq(&self, other: &VAddr) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for VAddr {
    fn partial_cmp(&self, other: &VAddr) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for VAddr {
    fn cmp(&self, other: &VAddr) -> Ordering {
        self.0.cmp(&other.0)
    }
}
