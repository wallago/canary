use canary_core::{
    data::temp::Temp,
    error::{Error, Result},
};
use esp_hal::{
    Blocking,
    delay::Delay,
    uart::{RxError, TxError, Uart},
};
use log::{debug, info};

/// Query current concentrations.
const GET_CONC: [u8; 2] = [0x70, 0x90];

/// Length of the reply.
const FRAME_LEN: usize = 22;

/// How long to wait for a reply.
const REPLY_TIMEOUT_MS: u32 = 200;

/// How often to check the RX FIFO while waiting for a reply.
const POLL_MS: u32 = 5;

/// Parsing `RxError` to `Error`.
fn rx_error(e: RxError) -> Error {
    Error::UartRx(match e {
        RxError::FifoOverflowed => "fifo overflowed",
        RxError::GlitchOccurred => "glitch on rx",
        RxError::FrameFormatViolated => "framing",
        RxError::ParityMismatch => "parity",
        _ => "unknown",
    })
}

/// Offsets into the reply. The frame is a header, four big-endian `f32`s, two
/// big-endian `u16`s, then a checksum — 1 + 16 + 4 + 1 = 22 bytes.
mod offset {
    /// Temperature, hundredths of a degree Celsius, big-endian `u16`.
    ///
    /// Confirmed by frame length and checksum; the *meaning* of this field is
    /// still the reading that best fits observed values, so if a warmed sensor
    /// moves a different field, this constant is the only thing to change.
    pub(super) const TEMP_C_X100: usize = 17;

    /// The trailing checksum byte.
    pub(super) const CHECKSUM: usize = 21;
}

/// Verify the trailing checksum: the sum of every preceding byte, including
/// the header, negated. Confirmed against both a datasheet frame and a live
/// capture.
fn checksum_ok(frame: &[u8; FRAME_LEN]) -> bool {
    let sum = frame[..offset::CHECKSUM]
        .iter()
        .fold(0_u8, |acc, &byte| acc.wrapping_add(byte));

    sum.wrapping_neg() == frame[offset::CHECKSUM]
}

/// The sensor's serial link.
pub(crate) struct Sensor<'d> {
    /// The UART the sensor hangs off.
    uart: Uart<'d, Blocking>,
    /// Used to pace the wait for a reply.
    delay: Delay,
}

impl<'d> Sensor<'d> {
    /// Wraps an already-configured UART.
    pub(crate) fn new(uart: Uart<'d, Blocking>) -> Self {
        Self {
            uart,
            delay: Delay::new(),
        }
    }

    /// Send the query and return the temperature it reports.
    ///
    /// `Ok(None)` covers the ordinary misses — no reply, a short reply, a bad
    /// checksum — since none of those are worth stopping the widget over.
    pub(crate) fn poll(&mut self) -> Result<Option<Temp>> {
        self.drain()?;
        self.request()?;

        let mut frame = [0u8; FRAME_LEN];
        let n = self.read_frame(&mut frame)?;

        if n == 0 {
            debug!("sensor: no reply");
            return Ok(None);
        }

        if n < FRAME_LEN {
            debug!(
                "sensor: short reply, {n} of {FRAME_LEN} bytes {:02X?}",
                &frame[..n]
            );
            return Ok(None);
        }

        if !checksum_ok(&frame) {
            debug!("sensor: bad checksum {frame:02X?}");
            return Ok(None);
        }

        let raw = u16::from_be_bytes([frame[offset::TEMP_C_X100], frame[offset::TEMP_C_X100 + 1]]);
        let celsius = f32::from(raw) / 100.0;

        info!("sensor: {celsius:.2}C from {frame:02X?}");

        Ok(Some(Temp::new(celsius)))
    }

    /// Drop anything left from an earlier exchange.
    fn drain(&mut self) -> Result<()> {
        let mut scratch = [0u8; FRAME_LEN];

        // Read it as keep calling read_buffered until it reports zero.
        while self.uart.read_buffered(&mut scratch).map_err(rx_error)? > 0 {}

        Ok(())
    }

    /// Send the query, and wait for it to actually leave the TX FIFO.
    fn request(&mut self) -> Result<()> {
        // `write` is allowed to take less than the whole slice.
        let mut sent = 0;
        while sent < GET_CONC.len() {
            sent += self
                .uart
                .write(&GET_CONC[sent..])
                .map_err(|_| Error::UartTx("write"))?;
        }
        self.uart.flush().map_err(|_| Error::UartTx("flush"))?;

        debug!("sensor: sent {GET_CONC:02X?}");
        Ok(())
    }

    /// Read frame, fill result and return its this.
    fn read_frame(&mut self, frame: &mut [u8; FRAME_LEN]) -> Result<usize> {
        let mut have = 0;
        let mut waited = 0;

        // Do it until expected lenght as been reach.
        while have < FRAME_LEN {
            // Fill frame content and get frame lenght.
            let n = self
                .uart
                .read_buffered(&mut frame[have..])
                .map_err(rx_error)?;

            // Wait a bit for next frame.
            if n == 0 {
                if waited >= REPLY_TIMEOUT_MS {
                    break;
                }
                self.delay.delay_millis(POLL_MS);
                waited += POLL_MS;
                continue;
            }

            // Add frames to reach the expected lenght.
            have += n;
        }

        Ok(have)
    }
}
