#![no_std]

mod draw;
mod screen;

use crate::screen::Framebuffer;
use panic_halt as _;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const NUMBER_OF_PIXELS: usize = WIDTH * HEIGHT;
pub const BYTES_PER_PIXEL: usize = 2;

#[no_mangle]
pub static mut FRAME: Framebuffer<WIDTH, HEIGHT, NUMBER_OF_PIXELS> =
    Framebuffer::<WIDTH, HEIGHT, NUMBER_OF_PIXELS>::new();

/// Pico DVI needs access to
#[no_mangle]
pub extern "C" fn framebuffer_get_raw_pointer() -> *mut u16 {
    unsafe { FRAME.data.as_mut_ptr() }
}
