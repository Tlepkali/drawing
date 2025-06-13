#![cfg(feature = "pixels-example")]
use drawing::seat_map::SeatMap;
use drawing::geometrical_shapes::Point;
use drawing::simple_image::Image;
use pixels::{Pixels, SurfaceTexture};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Seat map (pixels)").build(&event_loop).unwrap();
    let window_size = window.inner_size();
    let mut pixels = Pixels::new(window_size.width, window_size.height, SurfaceTexture::new(window_size.width, window_size.height, &window)).unwrap();
    let mut seat_map = SeatMap::new(5, 5, &Point::new(10, 10), 40, 5);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                let mut img = Image { width: window_size.width as i32, height: window_size.height as i32, data: frame.to_vec() };
                seat_map.draw(&mut img);
                frame.copy_from_slice(&img.data);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                pixels.resize_surface(size.width, size.height);
            }
            _ => {}
        }
    });
}

