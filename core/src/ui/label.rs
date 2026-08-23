use core::fmt::{self, Write};

/// A short string built on the stack.
pub struct Label {
    /// Bytes written so far. Only `..len` is meaningful.
    buf: [u8; Label::CAP],
    /// How much of `buf` is in use.
    len: usize,
}

impl Label {
    /// Capacity in bytes. One line of a reading, not a paragraph.
    pub const CAP: usize = 40;

    /// An empty label.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buf: [0; Self::CAP],
            len: 0,
        }
    }

    /// Format `args` into a new label.
    ///
    /// Anything that would overflow [`Label::CAP`] is dropped rather than
    /// truncated mid-character, leaving what was written before it.
    #[must_use]
    pub fn from_args(args: fmt::Arguments) -> Self {
        let mut label = Self::new();
        // The only failure is running out of room, and the partial result is
        // still drawable, so there is nothing useful to propagate.
        let _ = label.write_fmt(args);
        label
    }

    /// What has been written, as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // Every write goes through `write_str`, which only ever appends whole
        // `&str`s, so the buffer always holds valid UTF-8. The fallback keeps
        // this panic-free without an `unwrap`.
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("?")
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Label").field(&self.as_str()).finish()
    }
}

impl Write for Label {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Refuse rather than truncate: half a UTF-8 sequence is not a string.
        if bytes.len() > Self::CAP - self.len {
            return Err(fmt::Error);
        }

        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();

        Ok(())
    }
}
