use esp_hal::peripherals::Peripherals;
use esp_hal::{clock::CpuClock, delay::Delay, main};
use log::info;

use crate::board::Board;

pub(crate) fn run(peripherals: Peripherals) -> ! {
    /// Initialized display.
    let mut board = match crate::board::init_panel(peripherals) {
        Ok(board) => board,
        Err(e) => panic!("board init failed: {e}"),
    };

    /// Display simple Hello.
    canary_core::ui::hello(&mut board.display).unwrap_or_else(|e| panic!("draw: {e:?}"));

    // Busy-waits on the cycle counter rather than sleeping: it needs no
    // scheduler and no interrupts, so it works this early in boot.
    let delay = Delay::new();

    // The sensor answers a query rather than streaming, so the byte goes out
    // once per reading. The settle gives the reply time to land in the RX FIFO
    // before `read_buffered` looks — that call returns whatever is there now
    // and never waits.
    loop {
        if let Err(e) = board.sensor.poll() {
            info!("sensor: {e:?}");
        }

        delay.delay_millis(1_000);
    }
}
