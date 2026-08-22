//! Error handler implementation.

use thiserror::Error as ThisError;

/// Errors that can occur while running `canary`.
#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    /// A peripheral could not be brought up.
    #[error("Init error: `{0}`")]
    Init(&'static str),
    /// A bus refused the requested configuration (bad pin, unreachable clock).
    #[error("Bus error: `{0}`")]
    Bus(&'static str),
    /// The DMA descriptor/buffer pair was rejected.
    #[error("DMA error: `{0}`")]
    Dma(&'static str),
    /// The display did not act as expected.
    #[error("Display error: `{0}`")]
    Display(&'static str),
    /// The panel did not come up: reset or the init sequence failed.
    #[error("Panel error")]
    Panel,
    /// The UART reported a framing, parity, or overflow error.
    #[error("Uart RX error: `{0}`")]
    UartRx(&'static str),
    /// The command could not be pushed out of the TX FIFO.
    #[error("Uart TX error: `{0}`")]
    UartTx(&'static str),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_error() {
        let error = Error::Init("display");
        assert_eq!("Init error: `display`", error.to_string());
    }
}
