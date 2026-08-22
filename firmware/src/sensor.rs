use canary_core::error::{Error, Result};
use esp_hal::{
    Blocking,
    delay::Delay,
    uart::{RxError, TxError, Uart},
};
use log::{debug, info};

/// Query current concentrations: the command byte `0x70` followed by its
/// 0-add8 checksum. The sensor discards any frame whose checksum does not
/// match, which is why a bare `0x70` never gets a reply.
const GET_CONC: [u8; 2] = [0x70, 0x90];

/// Length of the `GET_CONC` reply: header, four gas readings, two environment
/// readings, checksum. Logged raw for now, not decoded.
const FRAME_LEN: usize = 22;

/// How long to wait for a reply. The 22 bytes take about 23 ms on the wire at
/// 9600 baud; the rest is the sensor's own turnaround.
const REPLY_TIMEOUT_MS: u32 = 200;

/// How often to check the RX FIFO while waiting for a reply.
const POLL_MS: u32 = 5;

/// `canary_core` knows nothing about esp-hal, so the concrete `RxError` has to
/// collapse to a static label on the way out. The variant is worth keeping:
/// framing and parity point at the wiring, an overflow at us.
fn rx_error(e: RxError) -> Error {
    Error::UartRx(match e {
        RxError::FifoOverflowed => "fifo overflowed",
        RxError::GlitchOccurred => "glitch on rx",
        RxError::FrameFormatViolated => "framing",
        RxError::ParityMismatch => "parity",
        _ => "unknown",
    })
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

    /// Send the query and log whatever comes back, raw. A short frame is
    /// logged as-is — during bring-up a partial reply is worth seeing.
    pub(crate) fn poll(&mut self) -> Result<()> {
        self.drain()?;
        self.request()?;

        let mut frame = [0u8; FRAME_LEN];
        let n = self.read_frame(&mut frame)?;

        if n == 0 {
            info!("sensor: no reply");
        } else if n < FRAME_LEN {
            info!(
                "sensor: short reply, {n} of {FRAME_LEN} bytes {:02X?}",
                &frame[..n]
            );
        } else {
            info!("sensor: {:02X?}", &frame[..n]);
        }

        Ok(())
    }

    /// Drop anything left from an earlier exchange, so a stale byte cannot be
    /// mistaken for the head of this reply.
    fn drain(&mut self) -> Result<()> {
        let mut scratch = [0u8; FRAME_LEN];

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

    /// Fill `frame`, or return however many bytes arrived before the deadline.
    /// `read_buffered` never blocks, so the pacing is ours to do.
    fn read_frame(&mut self, frame: &mut [u8; FRAME_LEN]) -> Result<usize> {
        let mut have = 0;
        let mut waited = 0;

        while have < FRAME_LEN {
            let n = self
                .uart
                .read_buffered(&mut frame[have..])
                .map_err(rx_error)?;

            if n == 0 {
                if waited >= REPLY_TIMEOUT_MS {
                    break;
                }
                self.delay.delay_millis(POLL_MS);
                waited += POLL_MS;
                continue;
            }

            have += n;
        }

        Ok(have)
    }
}
