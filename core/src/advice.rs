//! What the readings mean for the person at the desk: is the room
//! comfortable, and is there anything they should do about the air.
//!
//! The limits are first guesses from common indoor guidelines, not from the
//! X6 datasheet. Tune them here; the screen only shows the verdict.

/// Below this, the room is too cold, in degrees Celsius.
const TEMP_MIN: f32 = 19.0;
/// Above this, the room is too warm, in degrees Celsius.
const TEMP_MAX: f32 = 24.0;
/// Below this, the air is too dry, in percent relative humidity.
const HUMIDITY_MIN: f32 = 40.0;
/// Above this, the air is too humid, in percent relative humidity.
const HUMIDITY_MAX: f32 = 60.0;

/// Carbon monoxide alert level, in ppm (WHO 8-hour guideline, about 10 mg/m³).
const CO_MAX: f32 = 9.0;
/// Formaldehyde alert level, in ppm (WHO 0.1 mg/m³).
const HCHO_MAX: f32 = 0.08;
/// Total VOC alert level, in ppm.
const TVOC_MAX: f32 = 0.5;

/// How the room feels, worst problem first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comfort {
    /// Temperature and humidity are both in range.
    Comfortable,
    /// Below [`TEMP_MIN`].
    TooCold,
    /// Above [`TEMP_MAX`].
    TooWarm,
    /// Below [`HUMIDITY_MIN`].
    TooDry,
    /// Above [`HUMIDITY_MAX`].
    TooHumid,
}

/// Judge the room from temperature (°C) and relative humidity (%).
///
/// Temperature wins over humidity: it is the one people feel first.
#[must_use]
pub fn comfort(celsius: f32, humidity: f32) -> Comfort {
    if celsius < TEMP_MIN {
        Comfort::TooCold
    } else if celsius > TEMP_MAX {
        Comfort::TooWarm
    } else if humidity < HUMIDITY_MIN {
        Comfort::TooDry
    } else if humidity > HUMIDITY_MAX {
        Comfort::TooHumid
    } else {
        Comfort::Comfortable
    }
}

/// The one thing the screen should interrupt its rotation for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    /// Nothing to do.
    None,
    /// Carbon monoxide above [`CO_MAX`].
    Co,
    /// Formaldehyde above [`HCHO_MAX`].
    Hcho,
    /// Total VOCs above [`TVOC_MAX`].
    Tvoc,
    /// The sensor's own air quality index reads unhealthy or worse.
    Air,
}

/// Pick the alert to show, most dangerous first.
///
/// Gas readings are in ppm; `iaq` is the label the sensor module produces
/// (`"good"`, `"moderate"`, `"unhealthy"`, `"hazardous"`).
#[must_use]
pub fn alert(co: f32, hcho: f32, tvoc: f32, iaq: &str) -> Alert {
    if co > CO_MAX {
        Alert::Co
    } else if hcho > HCHO_MAX {
        Alert::Hcho
    } else if tvoc > TVOC_MAX {
        Alert::Tvoc
    } else if matches!(iaq, "unhealthy" | "hazardous") {
        Alert::Air
    } else {
        Alert::None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(21.0, 50.0, Comfort::Comfortable)]
    #[case(19.0, 40.0, Comfort::Comfortable)]
    #[case(24.0, 60.0, Comfort::Comfortable)]
    #[case(18.9, 50.0, Comfort::TooCold)]
    #[case(24.1, 50.0, Comfort::TooWarm)]
    #[case(21.0, 39.9, Comfort::TooDry)]
    #[case(21.0, 60.1, Comfort::TooHumid)]
    #[case(26.0, 30.0, Comfort::TooWarm)]
    fn test_comfort(#[case] celsius: f32, #[case] humidity: f32, #[case] expected: Comfort) {
        assert_eq!(expected, comfort(celsius, humidity));
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, "good", Alert::None)]
    #[case(0.0, 0.0, 0.0, "moderate", Alert::None)]
    #[case(9.0, 0.08, 0.5, "good", Alert::None)]
    #[case(0.0, 0.0, 0.0, "unhealthy", Alert::Air)]
    #[case(0.0, 0.0, 0.0, "hazardous", Alert::Air)]
    #[case(0.0, 0.0, 0.6, "good", Alert::Tvoc)]
    #[case(0.0, 0.1, 0.0, "good", Alert::Hcho)]
    #[case(10.0, 0.0, 0.0, "good", Alert::Co)]
    #[case(10.0, 0.1, 0.6, "hazardous", Alert::Co)]
    #[case(0.0, 0.1, 0.6, "hazardous", Alert::Hcho)]
    fn test_alert(
        #[case] co: f32,
        #[case] hcho: f32,
        #[case] tvoc: f32,
        #[case] iaq: &str,
        #[case] expected: Alert,
    ) {
        assert_eq!(expected, alert(co, hcho, tvoc, iaq));
    }
}
