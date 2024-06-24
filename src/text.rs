use crate::layout;
use ab_glyph::{point, Font, FontRef, Glyph, PxScale};

pub struct Label<'a> {
    font: FontRef<'a>,
    fontsize: f32,
    glyphs: Vec<Glyph>,
    pos: (f32, f32),
}

impl Label<'_> {
    pub fn new(text: String, pos: (f32, f32)) -> Self {
        let font =
            FontRef::try_from_slice(include_bytes!("/usr/share/fonts/TTF/OpenSans-Regular.ttf"))
                .unwrap();
        let fontsize = 45.0;
        let scale = PxScale::from(fontsize);
        let scaled_font = font.as_scaled(scale);

        let mut glyphs: Vec<Glyph> = Vec::new();

        layout::layout_paragraph(scaled_font, point(pos.0, pos.1), 9999.0, &text, &mut glyphs);

        Self {
            font,
            fontsize,
            glyphs,
            pos,
        }
    }

    pub fn set_text(&mut self, text: String) {
        let scale = PxScale::from(self.fontsize);
        let scaled_font = self.font.as_scaled(scale);
        self.glyphs.clear();

        layout::layout_paragraph(
            scaled_font,
            point(self.pos.0, self.pos.1),
            9999.0,
            &text,
            &mut self.glyphs,
        );
    }
}

pub trait Display {
    fn width(&self) -> u32;
    fn set_pixel(&mut self, x: u32, y: u32, color: u32);
}

pub fn draw(buffer: &mut dyn Display, label: &Label) {
    let m = label.fontsize as u32;
    let n = buffer.width() - m / 2;
    for glyph in &label.glyphs {
        let newline = glyph.position.x as u32 / n;
        if let Some(outlined) = label.font.outline_glyph(glyph.clone()) {
            let bounds = outlined.px_bounds();
            outlined.draw(|x, y, c| {
                //draw pixel `(x, y)` with coverage: `c`
                if c != 0.0 {
                    let bright = 255 - (c * 255.0) as u32;
                    buffer.set_pixel(
                        x + bounds.min.x as u32 - n * newline,
                        y + bounds.min.y as u32 + m * newline,
                        bright | (bright << 8) | (bright << 16),
                    );
                }
            });
        }
    }
}
