//! Pre-built canvas input event handlers.
//!
//! These are used with the `state` and `input` methods on the Canvas.

// @Todo: Write docs on how write your own input handler.

use crate::canvas::CanvasInfo;
/// Re-export the glutin module for writing your own event handlers.
pub use glium::glutin;
/// Re-export some common event types that are useful when writing your own
/// event handlers.
pub use glium::glutin::event::{Event, WindowEvent};

/// An input handler that tracks the position of the mouse.
///
/// It provides physical and virtual coordinates.
/// - Virtual coordinates (`virtual_x` and `virtual_y`) are as reported by the
///   OS, which means that they originate from the upper left corner and
///   account for DPI. You don't want this very often, but if you want to match
///   the OS coordinates for some reason, this is it.
/// - Physical coordinates (`x` and `y`) match the pixels in the image. This is
///   usually what you want.
pub struct MouseState {
    /// The x position from the lower-left corner, measured in physical pixels.
    /// This should always correspond to the column of the pixel in the image.
    pub x: i32,
    /// The y position from the lower-left corner, measured in physical pixels.
    /// This should always correspond to the row of the pixel in the image.
    pub y: i32,
    /// The x position from the upper-left corner as reported by the OS,
    /// measured in virtual pixels.
    pub virtual_x: i32,
    /// The y position from the upper-left corner as reported by the OS,
    /// measured in virtual pixels.
    pub virtual_y: i32,
}

impl MouseState {
    /// Create a MouseState. For use with the `state` method.
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            virtual_x: 0,
            virtual_y: 0,
        }
    }

    /// Handle input for the mouse. For use with the `input` method.
    pub fn handle_input(info: &CanvasInfo, mouse: &mut MouseState, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let (x, y): (i32, i32) = (*position).into();
                mouse.virtual_x = x;
                mouse.virtual_y = y;
                mouse.x = (x as f64 * info.dpi) as i32;
                mouse.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;
            }
            _ => (),
        }
    }
}
