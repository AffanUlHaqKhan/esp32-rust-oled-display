# esp32-rust-oled-display

Bare-metal Rust firmware for the ESP32-C6 that drives a 128x64 SSD1306 OLED over I2C and
renders text with `embedded-graphics`.

No RTOS, no `std` — `#![no_std]` / `#![no_main]` on top of [`esp-hal`](https://github.com/esp-rs/esp-hal) 1.1.

## Hardware

| Item | Value |
| --- | --- |
| MCU | ESP32-C6 (RISC-V, `riscv32imac-unknown-none-elf`) |
| Display | SSD1306 128x64, I2C |
| Bus | `I2C0` @ 400 kHz |
| I2C address | `0x3C` (the `ssd1306` crate default) |

### Wiring

| OLED pin | ESP32-C6 pin |
| --- | --- |
| SDA | GPIO21 |
| SCL | GPIO22 |
| VCC | 3V3 |
| GND | GND |

Pins are set in [src/bin/main.rs](src/bin/main.rs) via `.with_sda(...)` / `.with_scl(...)` — change
them there if your board is wired differently.

## Prerequisites

- Rust stable (>= 1.88). [rust-toolchain.toml](rust-toolchain.toml) pins the channel and pulls in
  the `riscv32imac-unknown-none-elf` target plus `rust-src` automatically.
- [`espflash`](https://github.com/esp-rs/espflash) for flashing and serial monitoring:

  ```bash
  cargo install espflash
  ```

- A USB connection to the board. On Linux your user needs access to the serial device
  (`sudo usermod -aG dialout $USER`, then re-login).

## Build and flash

`cargo run` is wired to `espflash flash --monitor` in [.cargo/config.toml](.cargo/config.toml), so
one command builds, flashes, and opens the serial monitor:

```bash
cargo run --release
```

Build only:

```bash
cargo build --release
```

Debug builds work too (`cargo run`) — the `dev` profile is compiled with `opt-level = "s"` because
unoptimized code is too slow and too large for the chip.

## What it does

1. Initializes the HAL with the CPU clock at maximum.
2. Brings up `I2C0` at 400 kHz on GPIO21/GPIO22.
3. Initializes the SSD1306 in buffered graphics mode, no rotation.
4. Draws `Hello Stranger !!!` at `(10, 32)` using the `FONT_6X10` mono font, then flushes the frame
   buffer to the panel.
5. Spins forever in a 1 s busy-wait loop.

Panics are logged through `esp-println` and then halt in a loop — see the `#[panic_handler]` in
[src/bin/main.rs](src/bin/main.rs).

## Logging

Log level comes from the `ESP_LOG` environment variable, defaulted to `info` in
[.cargo/config.toml](.cargo/config.toml). Override per run:

```bash
ESP_LOG=debug cargo run --release
```

## Project layout

```
.cargo/config.toml    target, runner (espflash), rustflags, ESP_LOG
build.rs              linker script wiring (linkall.x) + friendly linker diagnostics
rust-toolchain.toml   toolchain channel, target, components
src/bin/main.rs       firmware entry point
src/lib.rs            no_std crate root
```

## Troubleshooting

- **`Display init error` / `I2C init error`** — check wiring and pull-ups, and confirm the panel is
  at `0x3C`. Some modules ship strapped to `0x3D`; those need
  `I2CDisplayInterface::new_custom_address`.
- **Nothing on screen but no error** — the display was found on the bus but nothing was flushed;
  make sure `display.flush()` runs after drawing.
- **Flashing fails to find a port** — pass it explicitly: `espflash flash --monitor --chip esp32c6 -p /dev/ttyACM0 <elf>`.
- **`_stack_start` undefined at link time** — `linkall.x` is missing; `build.rs` must run (don't
  override `rustflags` in a way that drops the linker arg).

## License

Not yet specified.
