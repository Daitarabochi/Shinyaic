use crate::paint::font::PaintFont;
use crate::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use iced_graphics::Primitive;
use iced_native::{Background, Color, Point, Rectangle, Size};

pub fn create_block(color: Color, rect: RenderObjectRectangle) -> Primitive {
    Primitive::Quad {
        bounds: Rectangle::new(
            Point::new(rect.x, 45.0 + rect.y),
            Size::new(rect.width, rect.height),
        ),
        background: Background::Color(color),
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}

pub fn create_text(
    content: String,
    color: Color,
    rect: RenderObjectRectangle,
    font: PaintFont,
) -> Primitive {
    Primitive::Text {
        content,
        bounds: Rectangle::new(
            Point::new(rect.x, rect.y),
            Size::new(rect.width, rect.height),
        ),
        color: Color::from_rgba8(color.r as u8, color.g as u8, color.b as u8, color.a),
        size: font.size,
        font: font.font,
        horizontal_alignment: font.text_align,
        vertical_alignment: font.vertical_align,
    }
}
