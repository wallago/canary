//! Where things sit on the round panel.
//!
//! The framebuffer is a 360x360 square but only the inscribed circle is glass,
//! so a row's usable width shrinks as it moves away from the middle: 336 px
//! across the diameter, 226 px at y = 56, 102 px at y = 20. Every decision in
//! this module follows from that one fact.
//!
//! The panel is carved into three fixed bands, numbered as in the sketch:
//!
//! ```text
//!   ~~~~~~~~~~~~~~~~~~~~~~~~   y  20   zone 3 - info
//!        which screen am I
//!   ------------------------   y  72   zone 1 - primary
//!        headline reading
//!   ========================   y 180   the diameter: the widest row there is
//!        second reading             .. zone 2 - secondary
//!   ~~~~~~~~~~~~~~~~~~~~~~~~   y 340
//! ```
//!
//! The bands are fixed on purpose. A second screen varies what is drawn inside
//! them and the caption in [`Layout::info`] — not the geometry.

// Layout maths mixes `u32` extents (what `Size` and `Rectangle` want) with
// `i32` coordinates (what `Point` wants). Every value here is a pixel count on
// a 360x360 panel, so none of these casts can wrap; `try_from` on each one
// would only add panic paths to functions that cannot fail.
#![allow(clippy::cast_possible_wrap)]

use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Panel resolution. Square framebuffer, round glass.
pub const PANEL: Size = Size::new(360, 360);

/// Centre of the panel, and of the circle.
pub const CENTRE: Point = Point::new(180, 180);

/// Radius of the visible glass, in pixels.
pub const GLASS_RADIUS: u32 = 180;

/// Pixels held back from the glass edge. The outermost ring of a round panel is
/// where curvature and the bezel eat into legibility, so nothing lands there.
pub const BEZEL: u32 = 12;

/// Radius that content is allowed to reach.
pub const SAFE_RADIUS: u32 = GLASS_RADIUS - BEZEL;

/// Half the drawable width of row `y`, measured out from [`CENTRE`]. Zero for a
/// row that misses the circle entirely.
///
/// This is the honest answer for a single line of content, and the one to use
/// for anything near the top or bottom of the glass, where [`Zone::safe_area`]
/// collapses. Integer `isqrt`, because `core` has no `f32::sqrt` and pulling in
/// `libm` to place a caption would be a poor trade.
#[must_use]
pub fn half_width_at(y: i32) -> u32 {
    let dy = (y - CENTRE.y).unsigned_abs();

    if dy >= SAFE_RADIUS {
        return 0;
    }

    (SAFE_RADIUS * SAFE_RADIUS - dy * dy).isqrt()
}

/// Full drawable width of row `y`. See [`half_width_at`].
#[must_use]
pub fn width_at(y: i32) -> u32 {
    half_width_at(y) * 2
}

/// A horizontal band of the panel.
///
/// A band is a range of rows, not a rectangle — how much of each row is on
/// glass depends on how far that row sits from the middle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zone {
    /// First row of the band.
    pub top: i32,
    /// One past the last row of the band.
    pub bottom: i32,
}

impl Zone {
    /// Rows in the band.
    #[must_use]
    pub const fn height(&self) -> u32 {
        (self.bottom - self.top).unsigned_abs()
    }

    /// The whole band, ignoring the glass edge. For dividers and backgrounds
    /// that are meant to run off the side and be clipped by the bezel.
    #[must_use]
    pub const fn bounds(&self) -> Rectangle {
        Rectangle::new(
            Point::new(0, self.top),
            Size::new(PANEL.width, self.height()),
        )
    }

    /// The widest rectangle that fits inside the safe circle across *every* row
    /// of the band.
    ///
    /// Bound by whichever of the band's edges sits furthest from the middle, so
    /// it is pessimistic for a tall band and close to useless for one hard
    /// against the top or bottom of the glass — [`Layout::FIXED`]'s info band
    /// measures barely 100 px this way. Reach for [`width_at`] when a single
    /// line only needs its own row's width.
    #[must_use]
    pub fn safe_area(&self) -> Rectangle {
        let half = half_width_at(self.top).min(half_width_at(self.bottom - 1));

        Rectangle::new(
            Point::new(CENTRE.x - half as i32, self.top),
            Size::new(half * 2, self.height()),
        )
    }

    /// Centre of the band, for content centred on both axes.
    #[must_use]
    pub const fn centre(&self) -> Point {
        Point::new(CENTRE.x, self.top + (self.bottom - self.top) / 2)
    }
}

/// The fixed three-band frame every screen draws into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    /// Zone 3: top caption strip naming the screen you are looking at.
    pub info: Zone,
    /// Zone 1: the headline reading, between the caption and the diameter.
    pub primary: Zone,
    /// Zone 2: the second reading, below the diameter.
    pub secondary: Zone,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            info: Zone {
                top: 20,
                bottom: 72,
            },
            primary: Zone {
                top: 72,
                bottom: 180,
            },
            secondary: Zone {
                top: 180,
                bottom: 340,
            },
        }
    }
}
