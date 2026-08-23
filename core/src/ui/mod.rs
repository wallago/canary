use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        iso_8859_1::{FONT_6X10, FONT_10X20},
    },
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

/// The fixed decorative border.
pub mod border;

/// Where things sit on the round panel.
pub mod layout;

/// Building short strings without a heap.
pub mod label;

use layout::{CENTRE, Layout};

use crate::{data::temp::Temp, ui::label::Label};

/// The reading rides above the middle of zone 1, the verdict sits under it, so
/// the pair reads as one block rather than two stacked lines.
const VALUE_OFFSET: i32 = -12;

/// Offset of the verdict line from the middle of zone 1.
const VERDICT_OFFSET: i32 = 16;

/// The box wiped before each repaint. Wide enough for the longest verdict
/// (33 characters at 6 px), short enough that all four corners stay on glass.
const WIPE: Size = Size::new(280, 64);

/// Centred on both axes, so a zone's `centre` is the anchor.
fn centred() -> embedded_graphics::text::TextStyle {
    TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build()
}

/// Paint everything that never changes: background, chrome, fixed captions.
///
/// Draw once at start-up. [`reading`] repaints only the value on top of it,
/// which matters on an unbuffered panel where every redraw is visible.
///
/// # Errors
///
/// Propagates whatever the display returns.
pub fn frame<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let zones = Layout::default();

    display.clear(Rgb565::BLACK)?;
    border::draw(display)?;

    // Zone 3: which screen you are looking at.
    Text::with_text_style(
        "COMFORT",
        zones.info.centre(),
        MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE),
        centred(),
    )
    .draw(display)?;

    // The zone 1 / zone 2 rule sits on the diameter — the one row that spans a
    // full 336 px. `with_center` keeps the arithmetic in `Point` and `Size`, so
    // nothing here has to cast a width back into a coordinate.
    let split = zones.primary.bottom;
    Rectangle::with_center(
        Point::new(CENTRE.x, split),
        Size::new(layout::width_at(split), 1),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_DIM_GRAY))
    .draw(display)?;

    Ok(())
}

/// Repaint the temperature block: the reading with its unit, and the verdict
/// underneath.
///
/// Wipes only its own box, so the chrome [`frame`] drew survives untouched.
///
/// # Errors
///
/// Propagates whatever the display returns.
pub fn reading<D>(display: &mut D, temp: &Temp) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let middle = Layout::default().primary.centre();

    // Wipe first, or the old digits show through the new ones.
    Rectangle::with_center(middle, WIPE)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;

    let value = Label::from_args(format_args!("{:.1}{}", temp.value(), temp.unit()));

    Text::with_text_style(
        value.as_str(),
        middle + Point::new(0, VALUE_OFFSET),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
        centred(),
    )
    .draw(display)?;

    Text::with_text_style(
        temp.threshold(),
        middle + Point::new(0, VERDICT_OFFSET),
        MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_GRAY),
        centred(),
    )
    .draw(display)?;

    Ok(())
}
