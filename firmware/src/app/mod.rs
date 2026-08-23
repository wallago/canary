use canary_core::data::temp::Temp;
use esp_hal::peripherals::Peripherals;
use esp_hal::{clock::CpuClock, delay::Delay, main};
use log::info;

use crate::board::Board;

/// Everything the widget currently knows. One field today; the others land
/// here as their sensors are decoded.
#[derive(Default)]
struct Data {
    /// Last good temperature reading.
    temp: Temp,
}

pub(crate) fn run(peripherals: Peripherals) -> ! {
    /// Initialized display.
    let mut board = match crate::board::init_panel(peripherals) {
        Ok(board) => board,
        Err(e) => panic!("board init failed: {e}"),
    };

    let mut data = Data::default();
    // Chrome and captions never change, so they are painted once. The reading
    // goes up immediately after, showing the default until the first poll.
    canary_core::ui::frame(&mut board.display).unwrap_or_else(|e| panic!("draw: {e:?}"));
    canary_core::ui::reading(&mut board.display, &data.temp)
        .unwrap_or_else(|e| panic!("draw: {e:?}"));

    // Busy-waits on the cycle counter rather than sleeping: it needs no
    // scheduler and no interrupts, so it works this early in boot.
    let delay = Delay::new();

    // The sensor answers a query rather than streaming, so the byte goes out
    // once per reading. The settle gives the reply time to land in the RX FIFO
    // before `read_buffered` looks — that call returns whatever is there now
    // and never waits.
    loop {
        match board.sensor.poll() {
            Ok(Some(fresh)) => {
                // Repaint only on a change. The panel is unbuffered, so every
                // redraw is a visible flicker.
                if fresh != data.temp {
                    data.temp = fresh;
                    canary_core::ui::reading(&mut board.display, &data.temp)
                        .unwrap_or_else(|e| panic!("draw: {e:?}"));
                }
            }
            // A miss leaves the last good value on screen.
            Ok(None) => {}
            Err(e) => info!("sensor: {e:?}"),
        }

        delay.delay_millis(1_000);
    }
}
