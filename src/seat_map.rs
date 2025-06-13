use crate::geometrical_shapes::{Point, Rectangle, Drawable};
use raster::{Image, Color};

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
            0 => Color { r: 0, g: 255, b: 0, a: 255 },
            1 => Color { r: 0, g: 0, b: 255, a: 255 },
            _ => Color { r: 255, g: 0, b: 0, a: 255 },
        };
        self.rect.draw_filled_color(img, &color);
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
