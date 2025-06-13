use crate::geometrical_shapes::{Point, Line, Rectangle, Triangle, Circle};
use crate::simple_image::Color;
use std::fs::File;
use std::io::{self, Write};

pub struct SvgRenderer {
    width: i32,
    height: i32,
    elements: Vec<String>,
}

impl SvgRenderer {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height, elements: Vec::new() }
    }

    pub fn add_element(&mut self, element: String) {
        self.elements.push(element);
    }

    pub fn draw_line(&mut self, line: &Line, color: &Color) {
        let elem = format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"rgb({},{},{})\" stroke-width=\"1\" />",
            line.a.x, line.a.y, line.b.x, line.b.y, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn draw_point(&mut self, point: &Point, color: &Color) {
        let elem = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"1\" height=\"1\" fill=\"rgb({},{},{})\" />",
            point.x, point.y, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn draw_rectangle(&mut self, rect: &Rectangle, color: &Color) {
        let x = rect.a.x.min(rect.b.x);
        let y = rect.a.y.min(rect.b.y);
        let width = (rect.a.x - rect.b.x).abs();
        let height = (rect.a.y - rect.b.y).abs();
        let elem = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"rgb({},{},{})\" />",
            x, y, width, height, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn draw_filled_rectangle(&mut self, rect: &Rectangle, color: &Color) {
        let x = rect.a.x.min(rect.b.x);
        let y = rect.a.y.min(rect.b.y);
        let width = (rect.a.x - rect.b.x).abs();
        let height = (rect.a.y - rect.b.y).abs();
        let elem = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"rgb({},{},{})\" />",
            x, y, width, height, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn draw_triangle(&mut self, tri: &Triangle, color: &Color) {
        let elem = format!(
            "<polygon points=\"{},{} {},{} {},{}\" fill=\"none\" stroke=\"rgb({},{},{})\" />",
            tri.a.x, tri.a.y, tri.b.x, tri.b.y, tri.c.x, tri.c.y, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn draw_circle(&mut self, circle: &Circle, color: &Color) {
        let elem = format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"rgb({},{},{})\" />",
            circle.center.x, circle.center.y, circle.radius, color.r, color.g, color.b
        );
        self.add_element(elem);
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(
            file,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
            self.width, self.height
        )?;
        for e in &self.elements {
            writeln!(file, "    {}", e)?;
        }
        writeln!(file, "</svg>")?;
        Ok(())
    }
}
