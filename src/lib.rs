#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub use rtt_target;

use once_cell::sync::OnceCell;
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

static LOGGER: OnceCell<Logger> = OnceCell::new();

/// Init the logger with maximum level (Trace).
pub fn init() {
    init_with_level(log::LevelFilter::Trace);
}

/// Init the logger with a specific level.
pub fn init_with_level(level_filter: log::LevelFilter) {
    // Logger was already initialized.
    if LOGGER.get().is_some() {
        return;
    }
    let logger = LOGGER.get_or_init(|| Logger { level_filter });
    rtt_init_print!();

    // Use racy init if the feature is enabled or the target doesn't support atomic pointers.
    #[cfg(any(not(target_has_atomic = "ptr"), feature = "racy_init"))]
    unsafe {
        init_racy(logger);
    }

    // Use the default init otherwise.
    #[cfg(all(target_has_atomic = "ptr", not(feature = "racy_init")))]
    init_default(logger);
}

#[cfg(all(target_has_atomic = "ptr", not(feature = "racy_init")))]
fn init_default(logger: &'static Logger) {
    log::set_logger(logger).ok();
    log::set_max_level(logger.level_filter);
}

// # Safety
//
// This function will call the unsafe functions [log::set_logger_racy] and
// [log::set_max_level_racy] if either the feature `racy_init` is enabled or the target doesn't
// support atomic pointers. The [once_cell::OnceCell] should ensure that this is only called
// once.
#[cfg(any(not(target_has_atomic = "ptr"), feature = "racy_init"))]
unsafe fn init_racy(logger: &'static Logger) {
    log::set_logger_racy(logger).ok();
    log::set_max_level_racy(logger.level_filter);
}
