use crate::text::Display;

pub struct PixelBuffer<'a> {
    buffer: &'a mut [u32],
    height: u32,
    width: u32,
}

impl<'a> PixelBuffer<'a> {
    pub fn new(buffer: &'a mut [u32], height: u32, width: u32) -> PixelBuffer<'a> {
        Self {
            buffer,
            height,
            width,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }
}

impl Display for PixelBuffer<'_> {
    fn width(&self) -> u32 {
        self.width
    }
    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.buffer[(x + y * self.width) as usize] = color;
    }
}
