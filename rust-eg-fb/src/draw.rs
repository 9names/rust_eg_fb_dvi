use embedded_graphics::{
    // mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    primitives::{Circle, Line, Primitive, PrimitiveStyle, Rectangle, Triangle},
    // text::{Text, TextStyle, TextStyleBuilder},
};

use embedded_graphics_core::{geometry::Dimensions, prelude::*};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    text::LineHeight,
};
use embedded_text::{plugin::ansi::Ansi, style::TextBoxStyleBuilder, TextBox};

use crate::FRAME;

static CODESTR: &str = "[38;2;94;153;73m/// Comment
[97m#[[38;2;220;220;157mderive[97m(Debug)]
[38;2;84;128;166menum [38;2;78;201;176m[4mFoo[24m[97m<[38;2;84;128;166m'a[97m> {
[38;2;94;153;73m	/// Decide what [9mnot[29m to do next.
[48;5;235m	[38;2;36;144;241mBar[97m([38;2;78;201;176m[4mToken[24m[97m<[38;2;84;128;166m'a[97m>),[40C
[48;5;16m[97m}
";

#[no_mangle]
pub extern "C" fn framebuffer_draw() {
    static mut POS: i32 = 0;
    static mut INCREASING: bool = true;

    let display = unsafe { &mut FRAME };

    let _top_left = Point::new(0, 0);
    let _top_right = Point::new(320 - 1, 0);
    let bottom_left = Point::new(0, 240 - 1);
    let _bottom_right = Point::new(320 - 1, 240 - 1);
    let center_x = 320 / 2;
    let center_y = 240 / 2;
    let rect_height = 60;

    // Draw a couple of shapes to test for color/geometry
    Circle::new(Point::new(center_x - 20, unsafe { POS }), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)
        .unwrap();

    unsafe {
        if INCREASING {
            POS += 1;
            if POS == 239 {
                INCREASING = false;
            }
        } else {
            POS -= 1;
            if POS == 0 {
                INCREASING = true;
            }
        }
    }

    // Draw a couple of shapes to test for color/geometry
    Circle::new(Point::new(center_x - 20, unsafe { POS }), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_PURPLE))
        .draw(display)
        .unwrap();

    Rectangle::new(
        Point::new(0, center_y - (rect_height / 2)),
        Size::new(80, rect_height as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
    .draw(display)
    .unwrap();

    Rectangle::new(
        Point::new(320 - 80 - 1, center_y - (rect_height / 2)),
        Size::new(80, rect_height as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
    .draw(display)
    .unwrap();

    // Triangles require us to work things out ourselves - let's do some quick math
    // Isosceles are fun, lets do that!
    let tri_width = 60;
    let tri_height = 80;
    let tri_bottom_left = bottom_left + Point::new(center_x - (tri_width / 2), 0);
    let tri_bottom_right = bottom_left + Point::new(center_x + (tri_width / 2), 0);
    let tri_top = bottom_left + Point::new(center_x, -tri_height);

    Triangle::new(tri_bottom_left, tri_top, tri_bottom_right)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(display)
        .unwrap();

    // Draw some ANSI escape-code formatted text
    let text = CODESTR;

    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);
    let textbox_style = TextBoxStyleBuilder::new()
        .line_height(LineHeight::Percent(125))
        .build();

    TextBox::with_textbox_style(text, display.bounding_box(), character_style, textbox_style)
        .add_plugin(Ansi::new())
        .draw(display)
        .unwrap();

    // Add some diagonal lines from each corner to check we're painting to the edges
    let top_left = Point::new(0, 0);
    let top_right = Point::new(320 - 1, 0);
    let bottom_left = Point::new(0, 240 - 1);
    let bottom_right = Point::new(320 - 1, 240 - 1);

    let line_style = PrimitiveStyle::with_stroke(Rgb565::YELLOW, 1);
    Line::new(top_left, bottom_right)
        .into_styled(line_style)
        .draw(display)
        .unwrap();

    Line::new(bottom_left, top_right)
        .into_styled(line_style)
        .draw(display)
        .unwrap();
}
