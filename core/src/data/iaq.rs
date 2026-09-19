/// A IAQ reading in degrees Celsius.
#[derive(Default, PartialEq)]
pub struct IAQ<'a>(&'a str);

impl<'a> IAQ<'a> {
    /// Wrap a reading in degrees Celsius.
    #[must_use]
    pub const fn new(quality: &'a str) -> Self {
        Self(quality)
    }

    /// The reading in degrees Celsius.
    #[must_use]
    pub const fn value(&self) -> &'a str {
        self.0
    }
}
