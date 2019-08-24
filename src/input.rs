use crate::canvas::CanvasInfo;
use glium::glutin::event::{Event, WindowEvent};

pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub physical_pixels: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            physical_pixels: false,
        }
    }

    pub fn physical() -> Self {
        Self {
            x: 0,
            y: 0,
            physical_pixels: true,
        }
    }

    pub fn handle(info: &CanvasInfo, mouse: &mut MouseState, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let (x, y): (i32, i32) = (*position).into();
                mouse.x = x;
                mouse.y = info.height as i32 - y;
                if mouse.physical_pixels {
                    mouse.x = (mouse.x as f64 * info.dpi) as i32;
                    mouse.y = (mouse.y as f64 * info.dpi) as i32;
                }
            }
            _ => (),
        }
    }
}
