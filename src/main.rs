mod geometrical_shapes;
mod seat_map;
mod simple_image;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use simple_image::{Color, Image};
use seat_map::SeatMap;


fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new (
            &gs::Point::new(500, 500),
            &gs::Point::new(250, 700),
            &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    let start = gs::Point::new(100, 100);
    let seatmap = SeatMap::new(5, 10, &start, 15, 5);
    seatmap.draw(&mut image);

    simple_image::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color);
        }
    }
}