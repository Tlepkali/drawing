use crate::simple_image::{Color, Image};
use rand::Rng;

pub trait Drawable{
    fn draw(&self, img: &mut Image);
}

pub trait Displayable{
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Clone, Copy)]
pub struct Point{
    pub x: i32,
    pub y: i32,
}


impl Point{
    pub fn new(x1:i32,y1:i32)->Self{
        return Point{x:x1,y:y1}
    }
    pub fn random(x:i32,y:i32)->Self{
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0..x);
        let y1 = rng.gen_range(0..y);
        return Point::new(x1,y1)
    }
}

impl Drawable for Point{
    fn draw(&self,img: &mut Image){
        if self.x <= img.width && self.y <= img.height && self.x >= 0 && self.y >= 0{
            let color:Color = color(255,255,255);
            img.display(self.x, self.y, color)
        }
    }
}

pub fn color(r:u8, g:u8, b:u8) -> Color{
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0..r);
    let g1 = rng.gen_range(0..g);
    let b1 = rng.gen_range(0..b);
    return Color{r:r1,g:g1,b:b1,a:255}
}

#[derive(Debug, Clone, Copy)]
pub struct Line{
    pub a: Point,
    pub b: Point
}

impl Line{
    pub fn new(a: &Point,b:&Point) -> Line{
        Line{a:*a, b:*b}
    }
    pub fn random(x:i32,y:i32) -> Line{
        let a:Point = Point::random(x,y);
        let b:Point = Point::random(x,y);
        Line::new(&a,&b)
    }
}

impl Drawable for Line{
    fn draw(&self,img: &mut Image){
        let color:Color = color(255,255,255);
        line_algorithm(&self, img, &color)
    }
}

pub fn line_algorithm(l:&Line,img:&mut Image,color:&Color){
        let x0 = l.a.x;
        let y0 = l.a.y;
        let x1 = l.b.x;
        let y1 = l.b.y;
     // Create local variables for moving start point
    
        let mut x0 = x0;
        let mut y0 = y0;
    
        // Get absolute x/y offset
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };
    
        // Get slopes
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
    
        // Initialize error
        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err2;
    
        loop {
            // Set pixel
            //img.get_pixel_mut(x0 as u32, y0 as u32).data = [255, 255, 255];
            img.display(x0, y0, color.clone());

            // Check end condition
            if x0 == x1 && y0 == y1 { break };
    
            // Store old error
            err2 = 2 * err;
    
            // Adjust error and start position
            if err2 > -dx { err -= dy; x0 += sx; }
            if err2 < dy { err += dx; y0 += sy; }
        }
}

pub struct Triangle{
    pub a:Point,
    pub b:Point,
    pub c:Point
}

impl Triangle{
    pub fn new(a:&Point,b:&Point,c:&Point)->Self{
        Self{a:*a,b:*b,c:*c}
    }
}

impl Drawable for Triangle{
    fn draw(&self, img: &mut Image) {
        let l1 = Line{a:self.a,b:self.b};
        let l2 = Line{a:self.b,b:self.c};
        let l3 = Line{a:self.c,b:self.a};
        let color:Color = color(255,255,255);
        line_algorithm(&l1, img, &color);
        line_algorithm(&l2, img, &color);
        line_algorithm(&l3, img, &color);
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle{
    pub a:Point,
    pub b:Point,
    pub c:Point,
    pub d:Point
}

impl Rectangle{
    pub fn new(a:&Point,b:&Point)->Self{
        let c = Point{x:a.x,y:b.y};
        let d= Point{x:b.x,y:a.y};
        return Rectangle { a: *a, b: *b, c: c, d: d }
    }
    pub fn draw(&self,img:&mut Image){
        let l1 = Line{a:self.a,b:self.d};
        let l2 = Line{a:self.d,b:self.b};
        let l3 = Line{a:self.b,b:self.c};
        let l4 = Line{a:self.c,b:self.a};
        let color:Color = color(255,255,255);
        line_algorithm(&l1, img, &color);
        line_algorithm(&l2, img, &color);
        line_algorithm(&l3, img, &color);
        line_algorithm(&l4, img, &color);
    }

    pub fn draw_filled_color(&self, img:&mut Image, color:&Color){
        let left = self.a.x.min(self.b.x);
        let right = self.a.x.max(self.b.x);
        let top = self.a.y.min(self.b.y);
        let bottom = self.a.y.max(self.b.y);
        for x in left..right {
            for y in top..bottom {
                img.display(x, y, color.clone());
            }
        }
    }
}

pub struct Circle{
    pub center:Point,
    pub radius:i32
}

impl Circle{
    pub fn new(c:&Point,radius:i32)->Self{
        Self{
            center : *c,
            radius : radius
        }
    }
    pub fn drawoffset(img:&mut Image,xc:i32, yc:i32,x:i32, y:i32,color:&Color){
        img.display(xc+x, yc+y, color.clone());
        img.display(xc-x, yc+y, color.clone());
        img.display(xc+x, yc-y, color.clone());
        img.display(xc-x, yc-y, color.clone());
        img.display(xc+y, yc+x, color.clone());
        img.display(xc-y, yc+x, color.clone());
        img.display(xc+y, yc-x, color.clone());
        img.display(xc-y, yc-x, color.clone());
    }
    pub fn random(x:i32,y:i32)->Self{
        let center = Point::random(x, y);
        let mut rng = rand::thread_rng();
        let dif_x = x - center.x;
        let dif_y = y - center.y;
        let dif;
        let radius:i32;
        if dif_x > dif_y{
            dif = dif_y;
        }else{
            dif = dif_x;
        }
        if dif <= 2{
            radius = rng.gen_range(0..dif);
        }else{
            radius = rng.gen_range(2..dif);
        }
        Circle { center: center, radius: radius }
    }
}

impl Drawable for Circle{
    fn draw(&self, img: &mut Image) {
        let color:Color = color(255,255,255);
        let mut x = 0;
        let mut y = self.radius;
        let mut d = 3-2*self.radius;
        Circle::drawoffset(img, self.center.x, self.center.y, x, y,&color);
        while y >= x{
            x += 1;
            if d > 0{
                y -= 1;
                d = d + 4 * (x - y) + 3;
            }else{
                d = d + 4 * x + 2;
            }
            Circle::drawoffset(img, self.center.x, self.center.y, x, y,&color);
        }
    }
}