#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(not(any(feature = "cortex-m", feature = "riscv")))]
compile_error!("Either the `cortex-m` or `riscv` platform feature must be enabled.");

use rtt_target::{rprintln, rtt_init_print};

struct Logger {
    level_filter: log::LevelFilter,
}

impl log::Log for Logger {
    /// Returns if logger is enabled.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    /// Log the record.
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            rprintln!(
                "{:<5} [{}] {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    /// Flush buffered records.
    fn flush(&self) {
        // Nothing to do here
    }
}

static mut LOGGER: Logger = Logger {
    level_filter: log::LevelFilter::Trace,
};

/// Init the logger with maximum level (Trace).
pub fn init() {
    rtt_init_print!();
    unsafe {
        log::set_logger(&LOGGER).ok();
        log::set_max_level(LOGGER.level_filter);
    }
}

/// Init the logger with a specific level.
pub fn init_with_level(level_filter: log::LevelFilter) {
    unsafe {
        LOGGER.level_filter = level_filter;
    }
    init();
}
