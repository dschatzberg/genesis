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
use core::fmt::{self, Write};
use log;
use spin;
use super::console;

pub struct Logger {
    writer: spin::Mutex<LogWriter>,
}

impl Logger {
    pub fn init() {
        assert_has_not_been_called!("Logger::init() function \
                                    must only be called once");
        unsafe {
            let _ = log::set_logger_raw(|max_log_level| {
                static LOGGER: Logger =
                    Logger { writer: spin::Mutex::new(LogWriter) };
                max_log_level.set(log::LogLevelFilter::Debug);
                &LOGGER
            });
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            let mut w = self.writer.lock();
            let _ = writeln!(w,
                             "{}:{}: {}",
                             record.level(),
                             record.location().module_path(),
                             record.args());
        }
    }
}

struct LogWriter;

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { console::write_str(s) };
        Ok(())
    }
}
