#![cfg(feature = "pixels-example")]
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        event_loop
            .create_window(Window::default_attributes().with_title("Pixels Canvas"))
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

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        let _ = pixels.resize_surface(size.width, size.height);
                    }
                    WindowEvent::RedrawRequested => {
                        for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                            pixel.copy_from_slice(&[0x20, 0x20, 0x20, 0xff]);
                        }
                        let _ = pixels.render();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
