use canary_core::{
    data::{co::CO, hcho::HCHO, humidity::Humidity, iaq::IAQ, temp::Temp, tvoc::TVOC},
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

mod offset {
    pub(super) const IAQ: usize = 1;
    pub(super) const TVOC: usize = 5;
    pub(super) const HCHO: usize = 9;
    pub(super) const CO: usize = 13;
    pub(super) const TEMP: usize = 17;
    pub(super) const HUM: usize = 19;
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

fn float_at(frame: &[u8; FRAME_LEN], at: usize) -> f32 {
    f32::from_be_bytes([frame[at], frame[at + 1], frame[at + 2], frame[at + 3]])
}

fn hundredths_at(frame: &[u8; FRAME_LEN], at: usize) -> f32 {
    f32::from(u16::from_be_bytes([frame[at], frame[at + 1]])) / 100.0
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
    pub(crate) fn poll(
        &mut self,
    ) -> Result<Option<(Temp, Humidity, CO, HCHO, TVOC, IAQ<'static>)>> {
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

        let raw_iaq = float_at(&frame, offset::IAQ);
        let iaq = if raw_iaq <= 50.0 {
            "good"
        } else if raw_iaq > 50.0 && raw_iaq <= 100.0 {
            "moderate"
        } else if raw_iaq > 100.0 && raw_iaq <= 300.0 {
            "unhealthy"
        } else {
            "hazardous"
        };
        let tvoc = float_at(&frame, offset::TVOC);
        let hcho = float_at(&frame, offset::HCHO);
        let co = float_at(&frame, offset::CO);

        let celsius = hundredths_at(&frame, offset::TEMP);
        let humidity = hundredths_at(&frame, offset::HUM);

        debug!(
            "\nTemp: {celsius:.2}\nHum: {humidity}\nCO: {co}\nHCHO: {hcho}\nTVOC: {tvoc}\nIAQ: {iaq}"
        );

        Ok(Some((
            Temp::new(celsius),
            Humidity::new(humidity),
            CO::new(co),
            HCHO::new(hcho),
            TVOC::new(tvoc),
            IAQ::new(iaq),
        )))
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
