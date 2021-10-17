use crate::RGBA8;
use fontdue::layout::TextStyle;

pub struct Text<'a> {
    /// The text to layout.
    pub text: &'a str,
    /// The scale of the text in pixel units. The units of the scale are pixels per Em unit.
    pub px: f32,
    /// The font to layout the text in.
    pub font_index: usize,
    /// The text color,
    pub color: RGBA8,
}

impl<'a> Into<TextStyle<'a, RGBA8>> for &Text<'a> {
    fn into(self) -> TextStyle<'a, RGBA8> {
        TextStyle {
            text: self.text,
            px: self.px,
            font_index: self.font_index,
            user_data: self.color,
        }
    }
}
