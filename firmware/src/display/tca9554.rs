use esp_hal::{
    Blocking,
    delay::Delay,
    i2c::master::{Error, I2c},
};
use st77916::ResetInterface;

/// TCA9554 I/O expander I2C address.
const TCA9554_ADDR: u8 = 0x20;
/// TCA9554 register addresses.
const TCA9554_OUTPUT_REG: u8 = 0x01;
const TCA9554_CONFIG_REG: u8 = 0x03;
/// EXIO2 (bit index 1) drives the panel reset.
const EXIO_LCD_RST_BIT: u8 = 1 << 1;

/// Panel reset over the TCA9554 expander, implementing [`ResetInterface`].
pub(crate) struct TcaReset<'d> {
    pub i2c: I2c<'d, Blocking>,
    pub delay: Delay,
}

impl<'d> TcaReset<'d> {
    pub(crate) fn new(i2c: I2c<'d, Blocking>) -> Self {
        Self {
            i2c,
            delay: Delay::new(),
        }
    }

    /// Set or clear EXIO2 on the expander's OUTPUT register without disturbing
    /// the other pins.
    fn set_reset(&mut self, level: bool) -> Result<(), Error> {
        let mut out = [0u8];
        self.i2c
            .write_read(TCA9554_ADDR, &[TCA9554_OUTPUT_REG], &mut out)?;
        let val = if level {
            out[0] | EXIO_LCD_RST_BIT
        } else {
            out[0] & !EXIO_LCD_RST_BIT
        };
        self.i2c.write(TCA9554_ADDR, &[TCA9554_OUTPUT_REG, val])
    }
}

impl<'d> ResetInterface for TcaReset<'d> {
    type Error = Error;

    fn reset(&mut self) -> Result<(), Self::Error> {
        // Configure EXIO2 as an output (0 = output in the TCA9554 config reg).
        let mut cfg = [0u8];
        self.i2c
            .write_read(TCA9554_ADDR, &[TCA9554_CONFIG_REG], &mut cfg)?;
        self.i2c.write(
            TCA9554_ADDR,
            &[TCA9554_CONFIG_REG, cfg[0] & !EXIO_LCD_RST_BIT],
        )?;

        // Pulse reset low then high. Datasheet wants >=10us low and >=120ms
        // before commands; we are generous since this happens once at boot.
        self.set_reset(false)?;
        self.delay.delay_millis(20);
        self.set_reset(true)?;
        self.delay.delay_millis(120);
        Ok(())
    }
}
