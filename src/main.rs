mod geometrical_shapes;
mod seat_map;
mod simple_image;
mod svg_renderer;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use simple_image::{Color, Image};
use seat_map::SeatMap;
use svg_renderer::SvgRenderer;


fn main() {
    let mut image = Image::blank(1000, 1000);
    let mut svg = SvgRenderer::new(1000, 1000);

    let line = gs::Line::random(image.width, image.height);
    line.draw(&mut image);
    svg.draw_line(&line, &Color::rgba(255, 255, 255, 255));

    let point = gs::Point::random(image.width, image.height);
    point.draw(&mut image);
    svg.draw_point(&point, &Color::rgba(255, 255, 255, 255));

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);
    svg.draw_rectangle(&rectangle, &Color::rgba(255, 255, 255, 255));

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);
    svg.draw_triangle(&triangle, &Color::rgba(255, 255, 255, 255));

    for _ in 1..50 {
        let circle = gs::Circle::random(image.width, image.height);
        circle.draw(&mut image);
        svg.draw_circle(&circle, &Color::rgba(255, 255, 255, 255));
    }

    let start = gs::Point::new(100, 100);
    let seatmap = SeatMap::new(5, 10, &start, 15, 5);
    seatmap.draw(&mut image);
    seatmap.draw_svg(&mut svg);

    simple_image::save(&image, "image.png").unwrap();
    svg.save("image.svg").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color);
        }
    }
}