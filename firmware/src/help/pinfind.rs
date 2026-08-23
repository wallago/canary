use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    peripherals::Peripherals,
};
use log::info;

/// How many pins the scan watches.
const CANDIDATES: usize = 21;

/// How often to sample every pin.
const SCAN_MS: u32 = 200;

/// Watch every free GPIO and log whichever reads low. Never returns — this is
/// a bring-up mode, not part of the widget.
pub(crate) fn run(peripherals: Peripherals) -> ! {
    let cfg = InputConfig::default().with_pull(Pull::Up);

    // Everything the board is not already using. Left out: the QSPI panel
    // (21, 40-42, 45, 46), I2C (10, 11), the backlight (5), USB (19, 20), the
    // octal flash/PSRAM block (26-37), and BOOT (0).
    let pins: [(u8, Input<'static>); CANDIDATES] = [
        (1, Input::new(peripherals.GPIO1, cfg)),
        (2, Input::new(peripherals.GPIO2, cfg)),
        (3, Input::new(peripherals.GPIO3, cfg)),
        (4, Input::new(peripherals.GPIO4, cfg)),
        (6, Input::new(peripherals.GPIO6, cfg)),
        (7, Input::new(peripherals.GPIO7, cfg)),
        (8, Input::new(peripherals.GPIO8, cfg)),
        (9, Input::new(peripherals.GPIO9, cfg)),
        (12, Input::new(peripherals.GPIO12, cfg)),
        (13, Input::new(peripherals.GPIO13, cfg)),
        (14, Input::new(peripherals.GPIO14, cfg)),
        (15, Input::new(peripherals.GPIO15, cfg)),
        (16, Input::new(peripherals.GPIO16, cfg)),
        (17, Input::new(peripherals.GPIO17, cfg)),
        (18, Input::new(peripherals.GPIO18, cfg)),
        (38, Input::new(peripherals.GPIO38, cfg)),
        (39, Input::new(peripherals.GPIO39, cfg)),
        (43, Input::new(peripherals.GPIO43, cfg)),
        (44, Input::new(peripherals.GPIO44, cfg)),
        (47, Input::new(peripherals.GPIO47, cfg)),
        (48, Input::new(peripherals.GPIO48, cfg)),
    ];

    let delay = Delay::new();

    info!("pin scan: {CANDIDATES} pinsntify it");

    loop {
        for (number, pin) in &pins {
            if pin.is_low() {
                info!("pin scan: GPIO{number} is low");
            }
        }

        delay.delay_millis(SCAN_MS);
    }
}
