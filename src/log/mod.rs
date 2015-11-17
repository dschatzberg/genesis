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
use core::fmt::{self, Write};
use super::console;

#[macro_use]
pub mod macros;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };
        write!(f, "{}", s)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Record<'a> {
    pub location: Location,
    pub level: Level,
    pub args: fmt::Arguments<'a>,
    pub file: &'a str,
    pub line: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
}

struct Logger;

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { console::write_str(s) };
        Ok(())
    }
}

pub fn log(level: Level, loc: &'static Location, args: fmt::Arguments) {
    let _ = writeln!(Logger, "{}:{}: {}", level, loc.module_path, args);
}
