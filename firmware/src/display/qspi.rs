use esp_hal::{
    Blocking,
    spi::{
        Error,
        master::{Address, Command, DataMode, SpiDmaBus},
    },
};
use st77916::{
    ControllerInterface,
    commands::{QSPI_CMD_WRITE, QSPI_CMD_WRITE_4O, RAMWR, RAMWRC},
};

/// Largest pixel payload pushed in a single DMA transaction. Must be <= the DMA
/// TX buffer capacity allocated in `main`. One 360px RGB565 row is 720 bytes,
/// so this comfortably covers the per-row writes the unbuffered DrawTarget makes.
const PIXEL_CHUNK: usize = 4096;

/// QSPI transport implementing the driver's [`ControllerInterface`].
pub(crate) struct QspiInterface<'d> {
    pub bus: SpiDmaBus<'d, Blocking>,
    pub pixel_started: bool,
}

impl<'d> QspiInterface<'d> {
    /// QSPI register write: opcode 0x02, single-line address + data.
    fn reg_write(&mut self, cmd: u8, data: &[u8]) -> Result<(), Error> {
        self.bus.half_duplex_write(
            DataMode::Single,
            Command::_8Bit(QSPI_CMD_WRITE as u16, DataMode::Single),
            Address::_24Bit((cmd as u32) << 8, DataMode::Single),
            0,
            data,
        )
    }

    /// QSPI pixel write: opcode 0x32, single-line command/address, quad-line data.
    /// `ram_cmd` is RAMWR (first chunk) or RAMWRC (continuation).
    fn pixel_write(&mut self, ram_cmd: u8, data: &[u8]) -> Result<(), Error> {
        self.bus.half_duplex_write(
            DataMode::Quad,
            Command::_8Bit(QSPI_CMD_WRITE_4O as u16, DataMode::Single),
            Address::_24Bit((ram_cmd as u32) << 8, DataMode::Single),
            0,
            data,
        )
    }
}

impl<'d> ControllerInterface for QspiInterface<'d> {
    type Error = Error;

    fn send_command(&mut self, cmd: u8) -> Result<(), Self::Error> {
        self.pixel_started = false;
        self.reg_write(cmd, &[])
    }

    fn send_command_with_data(&mut self, cmd: u8, data: &[u8]) -> Result<(), Self::Error> {
        self.pixel_started = false;
        self.reg_write(cmd, data)
    }

    fn send_pixels(&mut self, pixels: &[u8]) -> Result<(), Self::Error> {
        // No byte-swap here: this panel reads RGB565 big-endian (high byte first),
        // which is exactly how the `st77916` crate already encodes pixels. The
        // reference drivers swap only because LVGL stores pixels little-endian;
        // we don't, so swapping would rotate the colour channels (green->red etc).
        for chunk in pixels.chunks(PIXEL_CHUNK) {
            let ram_cmd = if self.pixel_started { RAMWRC } else { RAMWR };
            self.pixel_write(ram_cmd, chunk)?;
            self.pixel_started = true;
        }
        Ok(())
    }
}
