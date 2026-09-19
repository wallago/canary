//! The widget: bring the board up, then keep the Slint screen in step with
//! the sensor.

use canary_core::{data::temp::Temp, prelude::*};
use esp_hal::{
    delay::Delay,
    peripherals::Peripherals,
    time::{Duration, Instant},
};
use log::{error, info};
use slint::ComponentHandle;

use crate::ui;

/// How long to leave between sensor queries.
const POLL_INTERVAL: Duration = Duration::from_millis(1_000);

/// Idle pause between frames when nothing is animating — roughly 60 Hz.
const FRAME_PAUSE_MS: u32 = 16;

/// Runs the widget. Never returns: there is no caller to return to.
pub(crate) fn run(peripherals: Peripherals) -> ! {
    let delay = Delay::new();

    if let Err(e) = widget(peripherals, delay) {
        // Nothing above this can act on the error. Report it and stop, leaving
        // whatever is on the glass where it is.
        error!("canary stopped: {e}");
    }

    loop {
        delay.delay_millis(1_000);
    }
}

/// Brings the board up and drives the screen.
///
/// Only returns on a fatal error: every driver has to stay alive, since
/// dropping an `Output` resets its pin and would take the backlight with it.
fn widget(peripherals: Peripherals, delay: Delay) -> Result<()> {
    let mut board = crate::board::init_panel(peripherals)?;

    let window = ui::init()?;
    let frame = ui::frame_buffer(&board.psram)?;
    let screen =
        ui::generated::EnvironmentDisplay::new().map_err(|_| Error::Init("slint screen"))?;
    screen.show().map_err(|_| Error::Init("slint show"))?;

    let mut temp = Temp::default();
    screen.set_temperature(temp.value());

    // `None` until the first query, so the first pass polls straight away
    // rather than subtracting the interval off a clock that has barely started.
    let mut polled: Option<Instant> = None;

    loop {
        slint::platform::update_timers_and_animations();

        if polled.is_none_or(|at| at.elapsed() >= POLL_INTERVAL) {
            polled = Some(Instant::now());

            match board.sensor.poll() {
                // Setting the property is what schedules a repaint; Slint
                // works out which lines changed and redraws only those.
                Ok(Some(fresh)) => {
                    if fresh != temp {
                        temp = fresh;
                        screen.set_temperature(temp.value());
                    }
                }
                // A miss leaves the last good value on screen.
                Ok(None) => {}
                Err(e) => info!("sensor: {e:?}"),
            }
        }

        ui::render(&window, &mut board.display, frame)
            .map_err(|_| Error::Display("render failed"))?;

        // Animations need frames; otherwise there is nothing to do until the
        // next poll, so idle instead of spinning on the bus.
        if !window.has_active_animations() {
            delay.delay_millis(FRAME_PAUSE_MS);
        }
    }
}
