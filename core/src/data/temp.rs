/// A temperature reading in degrees Celsius.
#[derive(Default, PartialEq)]
pub struct Temp(f32);

impl Temp {
    /// Wrap a reading in degrees Celsius.
    #[must_use]
    pub const fn new(celsius: f32) -> Self {
        Self(celsius)
    }

    /// The reading in degrees Celsius.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.0
    }
}
