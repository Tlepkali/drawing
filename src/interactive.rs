use crate::geometrical_shapes as gs;
use crate::simple_image::Image;
use crate::seat_map::SeatMap;
use crate::svg_renderer::SvgRenderer;
use gs::Drawable;
use std::io::{self, Write};

pub fn run() {
    let mut seat_map = SeatMap::new(5, 5, &gs::Point::new(10, 10), 20, 5);
    loop {
        println!("Commands: new <rows> <cols> <size> <padding> | set <row> <col> <tariff> | view | save <path> | exit");
        print!("editor> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "new" if parts.len() >= 5 => {
                if let (Ok(r), Ok(c), Ok(size), Ok(pad)) = (
                    parts[1].parse::<i32>(),
                    parts[2].parse::<i32>(),
                    parts[3].parse::<i32>(),
                    parts[4].parse::<i32>(),
                ) {
                    seat_map = SeatMap::new(r, c, &gs::Point::new(10, 10), size, pad);
                } else {
                    println!("Invalid numbers");
                }
            }
            "set" if parts.len() >= 4 => {
                if let (Ok(r), Ok(c), Ok(t)) = (
                    parts[1].parse::<i32>(),
                    parts[2].parse::<i32>(),
                    parts[3].parse::<u32>(),
                ) {
                    seat_map.set_tariff(r, c, t);
                } else {
                    println!("Invalid numbers");
                }
            }
            "view" => {
                seat_map.print_ascii();
            }
            "save" if parts.len() >= 2 => {
                let path = parts[1];
                let mut image = Image::blank(800, 600);
                let mut svg = SvgRenderer::new(800, 600);
                seat_map.draw(&mut image);
                seat_map.draw_svg(&mut svg);
                let _ = crate::simple_image::save(&image, &format!("{}.png", path));
                let _ = svg.save(&format!("{}.svg", path));
                println!("Saved to {}.png and {}.svg", path, path);
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
