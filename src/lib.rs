pub mod geometrical_shapes;
pub mod seat_map;
pub mod simple_image;
pub mod svg_renderer;
pub mod interactive;

use geometrical_shapes::Displayable;
use simple_image::{Image, Color};

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color);
        }
    }
}
