use embedded_graphics::pixelcolor::Rgb565;
use panic_halt as _;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    prelude::*,
};


#[repr(C, align(4))]
pub struct Framebuffer<const X: usize, const Y: usize, const BUFFERLEN: usize> {
    pub(crate) data: [u8; BUFFERLEN],
}

impl<const X: usize, const Y: usize, const BUFFERLEN: usize> Framebuffer<X, Y, BUFFERLEN> {
    pub const fn new() -> Framebuffer<X, Y, BUFFERLEN> {
        Framebuffer {
            data: [0; BUFFERLEN],
        }
    }

    pub fn pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        let offset = (x + y * X as u32) * 2;
        let bytes = color.to_le_bytes();
        self.data[offset as usize] = bytes[0];
        self.data[offset as usize + 1] = bytes[1];
    }

    pub fn clear(&mut self, color: Rgb565) {
        let bytes = color.to_le_bytes();
        for offset in (0..BUFFERLEN).step_by(2) {
            self.data[offset as usize] = bytes[0];
            self.data[offset as usize + 1] = bytes[1];
        }
    }
}

impl<const X: usize, const Y: usize, const BUFFERLEN: usize> DrawTarget
    for Framebuffer<X, Y, BUFFERLEN>
{
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();
        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| self.pixel(pos.x as u32, pos.y as u32, color));

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear(color);
        Ok(())
    }
}

impl<const X: usize, const Y: usize, const BUFFERLEN: usize> OriginDimensions
    for Framebuffer<X, Y, BUFFERLEN>
{
    fn size(&self) -> Size {
        Size::new(X as u32, Y as u32)
    }
}
