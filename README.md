# rtt-log

Log facade implementation for the Segger RTT protocol supported by the J-Link, ST-Link and other debug probes. It is based on [rtt-target](https://crates.io/crates/rtt-target).

> [!NOTE]
>
> `rtt-target` introduced log integration with version `0.6.1`, making this crate obsolete.
>
> Please refer to the [rtt-target docs](https://docs.rs/rtt-target/latest/rtt_target/#log-integration) for further details.

## Usage

```rust
// Init the logger with maximum level (Trace).
rtt_log::init();

// Alternatively, init the logger with specific level.
rtt_log::init_with_level(log::LevelFilter::Debug);

// Log something.
log::debug!("Application started");
```

Use a tool like [probe-rs](https://crates.io/crates/probe-rs) on the host to print the messages.

> [!NOTE]
>
> RTT uses a global symbol `_SEGGER_RTT` that can only appear once in a compiled binary. Therefore, if you want to use functions from `rtt-target` directly, import them from `rtt-log` instead of adding `rtt-target` as a separate dependency. Otherwise, a linker error about duplicate symbols will occur.

```rust
use rtt_log::rtt_target::rprintln;

rprintln!("Hello, world!");
```

## License

Published under the MIT license.

Author: Oliver Rockstedt <info@sourcebox.de>
