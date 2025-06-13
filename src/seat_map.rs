use crate::geometrical_shapes::{Point, Rectangle, Drawable};
use crate::simple_image::{Image, Color};
use crate::svg_renderer::SvgRenderer;

#[derive(Clone)]
pub struct Seat {
    rect: Rectangle,
    pub tariff: u32,
}

impl Seat {
    pub fn new(top_left: &Point, size: i32, tariff: u32) -> Self {
        let bottom_right = Point::new(top_left.x + size, top_left.y + size);
        let rect = Rectangle::new(top_left, &bottom_right);
        Self { rect, tariff }
    }
}

impl Drawable for Seat {
    fn draw(&self, img: &mut Image) {
        let color = match self.tariff {
            0 => Color::rgba(0, 255, 0, 255),
            1 => Color::rgba(0, 0, 255, 255),
            _ => Color::rgba(255, 0, 0, 255),
        };
        self.rect.draw_filled_color(img, &color);
    }
}

impl Seat {
    pub fn draw_svg(&self, svg: &mut SvgRenderer) {
        let color = match self.tariff {
            0 => Color::rgba(0, 255, 0, 255),
            1 => Color::rgba(0, 0, 255, 255),
            _ => Color::rgba(255, 0, 0, 255),
        };
        svg.draw_filled_rectangle(&self.rect, &color);
    }
}

pub struct SeatMap {
    pub seats: Vec<Seat>,
}

impl SeatMap {
    pub fn new(rows: i32, cols: i32, start: &Point, size: i32, padding: i32) -> Self {
        let mut seats = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                let x = start.x + c * (size + padding);
                let y = start.y + r * (size + padding);
                let tariff = r as u32 % 3;
                seats.push(Seat::new(&Point::new(x, y), size, tariff));
            }
        }
        Self { seats }
    }
}

impl Drawable for SeatMap {
    fn draw(&self, img: &mut Image) {
        for seat in &self.seats {
            seat.draw(img);
        }
    }
}

impl SeatMap {
    pub fn draw_svg(&self, svg: &mut SvgRenderer) {
        for seat in &self.seats {
            seat.draw_svg(svg);
        }
    }
}
