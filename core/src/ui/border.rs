use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyle},
};

use super::layout::CENTRE;

/// Centreline radius of the outer arc ring.
const ARC_RADIUS: u32 = 170;

/// Stroke width of the outer arc ring.
const ARC_WIDTH: u32 = 5;

/// Diameter of one dot in the inner ring.
const RING_DOT: u32 = 2;

/// How loudly an arc segment is drawn.
#[derive(Debug, Clone, Copy)]
enum Tone {
    /// The four long arcs that carry the shape.
    Bright,
    /// The short bridging arcs at 3 and 9 o'clock.
    Dim,
    /// Short ticks, drawn over a bridging arc.
    Alert,
}

/// One segment of the outer ring.
#[derive(Debug, Clone, Copy)]
struct Segment {
    /// Start angle in degrees, 0 at 3 o'clock, running clockwise.
    start: f32,
    /// Clockwise sweep in degrees.
    sweep: f32,
    /// How the segment is drawn.
    tone: Tone,
}

/// The outer ring, as an ordered list. Later entries draw over earlier ones.
///
/// The wide gaps at 12 and 6 o'clock are deliberate: that is where the caption
/// and the footer break the ring in the reference art.
#[rustfmt::skip]
const SEGMENTS: [Segment; 4] = [
    Segment { start: 282.0, sweep: 56.0, tone: Tone::Bright },
    Segment { start:  22.0, sweep: 56.0, tone: Tone::Bright },
    Segment { start: 102.0, sweep: 56.0, tone: Tone::Bright },
    Segment { start: 202.0, sweep: 56.0, tone: Tone::Bright },
    // Segment { start: 346.0, sweep: 32.0, tone: Tone::Dim    },
    // Segment { start: 166.0, sweep: 32.0, tone: Tone::Dim    },
    // Segment { start: 188.0, sweep:  4.0, tone: Tone::Alert  },
    // Segment { start: 195.0, sweep:  4.0, tone: Tone::Alert  },
];

/// One quadrant of the inner dot ring, as offsets from [`CENTRE`].
///
/// Only a quarter is stored. Rotating a point by 90 degrees is
/// `(x, y) -> (-y, x)`, which is exact in integers, so 30 entries yield all
/// 120 dots with no trigonometry — and `core` has no `sin`/`cos` to offer
/// anyway. Generated for radius 158 at 3 degree steps; every resulting point
/// lands within 0.6 px of that radius.
#[rustfmt::skip]
const RING_QUADRANT: [(i32, i32); 30] = [
    ( 158,    0), ( 158,    8), ( 157,   17), ( 156,   25), ( 155,   33),
    ( 153,   41), ( 150,   49), ( 148,   57), ( 144,   64), ( 141,   72),
    ( 137,   79), ( 133,   86), ( 128,   93), ( 123,   99), ( 117,  106),
    ( 112,  112), ( 106,  117), (  99,  123), (  93,  128), (  86,  133),
    (  79,  137), (  72,  141), (  64,  144), (  57,  148), (  49,  150),
    (  41,  153), (  33,  155), (  25,  156), (  17,  157), (   8,  158),
];

/// Draw the border chrome.
///
/// Fixed art: nothing here depends on a reading yet. Draw it once after
/// clearing, before any content.
pub fn draw<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    for segment in &SEGMENTS {
        let colour = match segment.tone {
            Tone::Bright => Rgb565::WHITE,
            Tone::Dim => Rgb565::CSS_DIM_GRAY,
            Tone::Alert => Rgb565::RED,
        };

        Arc::with_center(
            CENTRE,
            ARC_RADIUS * 2,
            segment.start.deg(),
            segment.sweep.deg(),
        )
        .into_styled(PrimitiveStyle::with_stroke(colour, ARC_WIDTH))
        .draw(display)?;
    }

    let dot = PrimitiveStyle::with_fill(Rgb565::CSS_GRAY);

    for &(x, y) in &RING_QUADRANT {
        // The four rotations of one stored point cover the whole ring.
        for offset in [
            Point::new(x, y),
            Point::new(-y, x),
            Point::new(-x, -y),
            Point::new(y, -x),
        ] {
            Circle::with_center(CENTRE + offset, RING_DOT)
                .into_styled(dot)
                .draw(display)?;
        }
    }

    Ok(())
}
