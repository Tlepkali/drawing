use drawing::seat_map::SeatMap;
use drawing::geometrical_shapes::{Point, Drawable};
use drawing::simple_image::Image;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        event_loop
            .create_window(Window::default_attributes().with_title("Drawing Preview"))
            .unwrap(),
    );

    let mut pixels = {
        let size = window.inner_size();
        Pixels::new(
            size.width,
            size.height,
            SurfaceTexture::new(size.width, size.height, window.clone()),
        )
        .unwrap()
    };

    let seat_map = SeatMap::new(5, 5, &Point::new(10, 10), 40, 5);

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(size) => {
                    let _ = pixels.resize_surface(size.width, size.height);
                }
                WindowEvent::RedrawRequested => {
                    let size = window.inner_size();
                    let mut buffer = Image {
                        width: size.width as i32,
                        height: size.height as i32,
                        data: pixels.frame().to_vec(),
                    };
                    seat_map.draw(&mut buffer);
                    pixels.frame_mut().copy_from_slice(&buffer.data);
                    let _ = pixels.render();
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
