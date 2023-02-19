# rtt-log

This Rust crate provides a log facade implementation for the Segger RTT protocol supported by the J-Link, ST-Link and other debug probes. It currently supports ARM Cortex-M and RISC-V targets via the [rtt-target](https://crates.io/crates/rtt-target) crate.

## Usage

Either the `cortex-m` or the `riscv` platform feature must be enabled when defining this crate dependency.

```rust
// Init the logger with maximum level (Trace).
rtt_log::init();

// Alternatively, init the logger with specific level.
rtt_log::init_with_level(log::LevelFilter::Debug);

// Log something.
log::debug!("Application started");
```

## Features

### cortex-m

Support for ARM Cortex-M targets. Disabled by default.

### riscv

Support for RISC-V targets. Disabled by default.

Use a tool like [probe-run](https://github.com/knurling-rs/probe-run) on the host to print the messages.

## License

Published under the MIT license.

Author: Oliver Rockstedt <info@sourcebox.de>
