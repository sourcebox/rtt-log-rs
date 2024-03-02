# rtt-log

This Rust crate provides a log facade implementation for the Segger RTT protocol supported by the J-Link, ST-Link and other debug probes. It currently supports ARM Cortex-M and RISC-V targets via the [rtt-target](https://crates.io/crates/rtt-target) crate.

## Usage

```rust
// Init the logger with maximum level (Trace).
rtt_log::init();

// Alternatively, init the logger with specific level.
rtt_log::init_with_level(log::LevelFilter::Debug);

// Log something.
log::debug!("Application started");
```

**Note:**

RTT uses a global symbol `_SEGGER_RTT` that can only appear once in a compiled binary. Therefore, if you want to use functions from `rtt-target` directly, import them from `rtt-log` instead of adding `rtt-target` as a separate dependency. Otherwise, a linker error about duplicate symbols will occur.

```rust
use rtt_log::rtt_target::rprintln;

rprintln!("Hello, world!");
```

Use a tool like [probe-run](https://github.com/knurling-rs/probe-run) on the host to print the messages.

## License

Published under the MIT license.

Author: Oliver Rockstedt <info@sourcebox.de>
