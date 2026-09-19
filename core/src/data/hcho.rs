/// A HCHO reading in degrees Celsius.
#[derive(Default, PartialEq)]
pub struct HCHO(f32);

impl HCHO {
    /// Wrap a reading in degrees Celsius.
    #[must_use]
    pub const fn new(gaz: f32) -> Self {
        Self(gaz)
    }

    /// The reading in degrees Celsius.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.0
    }
}
