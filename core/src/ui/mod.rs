use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

/// Centre of the 360x360 round panel. The corners are behind the bezel.
const CENTRE: Point = Point::new(180, 180);

/// Clear the screen and write `hello` in the middle of it.
pub fn hello<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    display.clear(Rgb565::BLACK)?;

    // Two separate styles, and the split confuses everyone at first:
    //   character style = what the glyphs look like (font, colour)
    //   text style      = where the string sits relative to `CENTRE`
    let character = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    let layout = TextStyleBuilder::new()
        .alignment(Alignment::Center)   // CENTRE.x is the middle, not the left edge
        .baseline(Baseline::Middle)     // CENTRE.y is the middle, not the baseline
        .build();

    Text::with_text_style("hello", CENTRE, character, layout).draw(display)?;

    Ok(())
}
