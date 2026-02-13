use log::{Level, Metadata, Record};

/// A simplistict logger.
pub struct Lugger;

impl log::Log for Lugger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_insert = match record.level() {
                Level::Error => format!("[{}]", record.level()),
                Level::Warn => format!("[{}]", record.level()),
                Level::Info => format!("[{}]", record.level()),
                Level::Debug => format!("[{}]", record.level()),
                Level::Trace => format!("[{}]", record.level()),
            };
            let target_insert = {
                let record_line = if record.line().is_some() {
                    format!("::{}", record.line().unwrap())
                } else {
                    format!("")
                };
                format!("{}{}", record.target(), record_line)
            };

            println!("{} - {} - {}", level_insert, target_insert, record.args());
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};

static LOGGER: Lugger = Lugger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
