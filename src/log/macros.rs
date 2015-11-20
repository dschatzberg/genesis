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

#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => ({
        static LOC: $crate::log::Location = $crate::log::Location {
            line: line!(),
            file: file!(),
            module_path: module_path!(),
        };
        let lvl = $lvl;
        $crate::log::log(lvl, &LOC, format_args!($($arg)+))
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (log!($crate::log::Level::Error, $($arg)*))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (log!($crate::log::Level::Warn, $($arg)*))
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (log!($crate::log::Level::Info, $($arg)*))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (log!($crate::log::Level::Debug, $($arg)*))
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => (log!($crate::log::Level::Trace, $($arg)*))
}
