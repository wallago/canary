use esp_hal::{
    Blocking,
    uart::{RxError, Uart},
};
use log::{debug, info};

/// Bytes drained per poll. Larger than any single frame we expect, so a whole
/// message is normally visible in one log line.
const READ_CHUNK: usize = 128;

/// The sensor's serial link.
pub(crate) struct Sensor<'d> {
    uart: Uart<'d, Blocking>,
}

impl<'d> Sensor<'d> {
    pub(crate) fn new(uart: Uart<'d, Blocking>) -> Self {
        Self { uart }
    }

    /// Log whatever the sensor has sent since the last call.
    pub(crate) fn log_pending(&mut self) -> Result<(), RxError> {
        let mut buf = [0u8; READ_CHUNK];
        let n = self.uart.read_buffered(&mut buf)?;

        if n == 0 {
            debug!("sensor: nothing");
            return Ok(());
        }

        info!("sensor: {n} bytes {:02X?}", &buf[..n]);
        Ok(())
    }
}
