use crate::text::Display;
use std::rc::Rc;

type Buffer<'a> = softbuffer::Buffer<'a, Rc<winit::window::Window>, Rc<winit::window::Window>>;

pub struct PixelBuffer<'a> {
    buffer: Buffer<'a>,
    height: u32,
    width: u32,
}

impl<'a> PixelBuffer<'a> {
    pub fn new(buffer: Buffer<'a>, height: u32, width: u32) -> Self {
        Self {
            buffer,
            height,
            width,
        }
    }

    pub fn buffer(self) -> Buffer<'a> {
        self.buffer
    }
}

impl PixelBuffer<'_> {
    pub fn clear(&mut self, color: u32) {
        //self.buffer.iter_mut().for_each(|m| *m = 0);
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
