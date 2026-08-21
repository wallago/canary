//! Firmware entry point for `canary`.

// No `std`: no heap, no files, no threads, no `std::time`. Only `core` (and
// `alloc`, if a global allocator is ever registered). Every crate in the
// dependency tree has to agree — one `std` crate anywhere and the link fails.
#![no_std]
// Suppress the normal Rust entry shim. On a hosted target, `fn main` is not
// really the entry point: the runtime sets up argv/env/stack first, then calls
// it. None of that exists here, so `#[esp_hal::main]` below emits the real
// entry symbol instead.
#![no_main]

// Imported purely for its linker side effect: esp-backtrace supplies the
// `#[panic_handler]` that every `no_std` binary must provide exactly once.
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, delay::Delay, main};
use log::info;

/// Main application.
mod app;

/// Screen hardware communication.
mod display;

/// UART sensor link.
mod sensor;

/// Board hardware config.
mod board;

// Emits a metadata struct into the `.flash.appdesc` section: image version,
// project name, build date, the IDF version this was built against. The
// second-stage bootloader reads it to identify and validate the app before
// jumping in.
esp_bootloader_esp_idf::esp_app_desc!();

/// Boots the board and runs the widget loop.
// The return type is `!` — not a style choice. There is no caller to return
// to; falling off the end of `main` would execute whatever bytes follow it.
#[main]
fn main() -> ! {
    // Registers the heap before anything can allocate. st77916's unbuffered
    // DrawTarget takes a row buffer per draw call; nothing else here allocates.
    esp_alloc::heap_allocator!(size: 16 * 1024);

    // Routes `log` macros to the UART / USB-Serial-JTAG console. The filter
    // comes from `ESP_LOG` in `.cargo/config.toml`, which esp-println's build
    // script bakes in at compile time — it is not read at runtime.
    esp_println::logger::init_logger_from_env();

    // Beyond raising the clock to 240 MHz, `init` disables the RTC and timer-
    // group watchdogs, so nothing reboots us if a loop stops making progress.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // Returns the peripheral singletons: one owned handle per hardware block,
    // handed out exactly once.
    let peripherals = esp_hal::init(config);

    info!("{} booting", canary_core::version!());

    /// Initialized display.
    let mut board = match board::init_panel(peripherals) {
        Ok(board) => board,
        Err(e) => panic!("board init failed: {e}"),
    };

    /// Display simple Hello.
    canary_core::ui::hello(&mut board.display).unwrap_or_else(|e| panic!("draw: {e:?}"));

    // Busy-waits on the cycle counter rather than sleeping: it needs no
    // scheduler and no interrupts, so it works this early in boot.
    let delay = Delay::new();

    // The "and now, forever" part. Every terminal path in a firmware binary
    // ends in a loop like this one, a reset, or a panic.
    loop {
        if let Err(e) = board.sensor.log_pending() {
            info!("sensor read error: {e:?}");
        }
        delay.delay_millis(1_000);
    }
}
