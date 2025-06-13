use image::{ImageError, ColorType};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn blank(width: i32, height: i32) -> Self {
        let size = (width * height * 4) as usize;
        Self { width, height, data: vec![0; size] }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        self.data[idx] = color.r;
        self.data[idx + 1] = color.g;
        self.data[idx + 2] = color.b;
        self.data[idx + 3] = color.a;
    }
}

pub fn save(image: &Image, path: &str) -> Result<(), ImageError> {
    image::save_buffer(
        path,
        &image.data,
        image.width as u32,
        image.height as u32,
        ColorType::Rgba8,
    )
}
