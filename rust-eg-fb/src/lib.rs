#![no_std]

mod draw;
mod screen;

use panic_halt as _;
use crate::screen::Framebuffer;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const BYTES_PER_PIXEL: usize = 2;
const BYTES_IN_BUFFER: usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;

#[no_mangle]
pub static mut FRAME: Framebuffer<WIDTH, HEIGHT, BYTES_IN_BUFFER> =
    Framebuffer::<WIDTH, HEIGHT, BYTES_IN_BUFFER>::new();

/// Pico DVI needs access to
#[no_mangle]
pub extern "C" fn framebuffer_get_raw_pointer() -> *mut u8 {
    unsafe { FRAME.data.as_mut_ptr() }
}
