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

    /// What this temperature does to a person trying to work in it.
    #[must_use]
    pub fn threshold(&self) -> &'static str {
        match self.0 {
            t if t.is_nan() => "invalid reading",
            t if t < 18.0 => "Cold hands, typing accuracy drops",
            t if t < 20.0 => "Acceptable in with a sweater",
            t if t < 26.0 => "Peak comfort and productivity",
            t if t < 28.0 => "Drowsiness, concentration drops",
            _ => "Measurable performance loss",
        }
    }

    /// The unit these readings are in.
    // Takes `self` despite not reading it: this is the shape the trait will
    // want when the other sensors arrive, and a free function would have to be
    // renamed then.
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn unit(&self) -> &'static str {
        "\u{b0}C"
    }
}
