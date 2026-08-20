use canary_core::prelude::*;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::RgbColor};
use esp_hal::{
    delay::Delay,
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
    spi::{
        Mode,
        master::{Config as SpiConfig, Spi},
    },
    time::Rate,
};

use crate::screen::{self, PanelDisplay};

/// Everything board bring-up produces.
pub struct Board {
    pub display: PanelDisplay<'static>,
    pub backlight: Output<'static>,
}

/// Wire up the buses and bring the panel to a drawable state.
pub(crate) fn init_panel(peripherals: Peripherals) -> Result<Board> {
    let backlight = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());

    let mut delay = Delay::new();

    // DMA needs buffers that live in DRAM.
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(8, 32768);
    let dma_rx = DmaRxBuf::new(rx_descriptors, rx_buffer).map_err(|_| Error::Dma("rx"))?;
    let dma_tx = DmaTxBuf::new(tx_descriptors, tx_buffer).map_err(|_| Error::Dma("tx"))?;

    // 40 MHz, SPI mode 0 (clock idles low, sample on the rising edge).
    let spi_config = SpiConfig::default()
        .with_frequency(Rate::from_mhz(40))
        .with_mode(Mode::_0);

    // What makes this QSPI rather than ordinary SPI: four bidirectional data
    // lines (SIO0..SIO3) instead of a MOSI/MISO pair. Pixel writes drive all
    // four at once, so a byte takes two clocks instead of eight.
    let bus = Spi::new(peripherals.SPI2, spi_config)
        .map_err(|_| Error::Bus("spi2"))?
        .with_sck(peripherals.GPIO40)
        .with_cs(peripherals.GPIO21)
        .with_sio0(peripherals.GPIO46)
        .with_sio1(peripherals.GPIO45)
        .with_sio2(peripherals.GPIO42)
        .with_sio3(peripherals.GPIO41)
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx, dma_tx);

    // The panel's reset line is not an MCU pin on this board — it hangs off
    // EXIO2 of the expander at address 0x20, so resetting the display means a
    // I2C write. 400 kHz is the expander's fast-mode rating.
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .map_err(|_| Error::Bus("i2c0"))?
    .with_scl(peripherals.GPIO10)
    .with_sda(peripherals.GPIO11);

    // Hands both buses to the driver, which pulses reset over I2C, runs the
    // ~150-entry init table, and leaves the controller awake with the display
    // on. After this returns, the value is a `DrawTarget`.
    let mut display = screen::build(bus, i2c, &mut delay).map_err(|_| Error::Panel)?;

    // First light. If the panel goes red, then the backlight, both buses, the
    // reset over I2C and the init table are all working, and anything that
    // goes wrong from here is in the drawing layer.
    display
        .clear(Rgb565::RED)
        .map_err(|_| Error::Display("clear failed"))?;

    Ok(Board { display, backlight })
}
