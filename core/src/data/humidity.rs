/// A Humidity reading in degrees.
#[derive(Default, PartialEq)]
pub struct Humidity(f32);

impl Humidity {
    /// Wrap a reading in degrees Celsius.
    #[must_use]
    pub const fn new(humidity: f32) -> Self {
        Self(humidity)
    }

    /// The reading in degrees Celsius.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.0
    }
}
