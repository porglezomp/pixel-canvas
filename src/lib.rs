#![deny(missing_docs)]

//! This crate is designed to make it easy to build interactive computer art
//! with just a pixel buffer. For inspiration, consider looking at
//! <https://shadertoy.com> and <http://www.iquilezles.org/www/index.htm>,
//! there are a lot of cool art pieces to see and explanations of fun techniques!
//!
//! # Usage
//!
//! To make a piece of art, you create and configure a [`Canvas`] object, and
//! then you ask it to [`render`] with your code. The canvas will do state
//! management and hand you an image to modify. Whatever modifications you make
//! to the image will be displayed on the screen.
//!
//! [`Canvas`]: struct.Canvas.html
//! [`render`]: struct.Canvas.html#method.render
//!
//! # Example
//!
//! ```rust,no_run
//! use pixel_canvas::{Canvas, Color, input::MouseState};
//!
//! fn main() {
//!     // Configure the window that you want to draw in. You can add an event
//!     // handler to build interactive art. Input handlers for common use are
//!     // provided.
//!     let canvas = Canvas::new(512, 512)
//!         .title("Tile")
//!         .state(MouseState::new())
//!         .input(MouseState::handle_input);
//!     // The canvas will render for you at up to 60fps.
//!     canvas.render(|mouse, image| {
//!         // Modify the `image` based on your state.
//!         let width = image.width() as usize;
//!         for (y, row) in image.chunks_mut(width).enumerate() {
//!             for (x, pixel) in row.iter_mut().enumerate() {
//!                 let dx = x as i32 - mouse.x;
//!                 let dy = y as i32 - mouse.y;
//!                 let dist = dx * dx + dy * dy;
//!                 *pixel = Color {
//!                     r: if dist < 128 * 128 { dy as u8 } else { 0 },
//!                     g: if dist < 128 * 128 { dx as u8 } else { 0 },
//!                     b: (x * y) as u8,
//!                 }
//!             }
//!         }
//!     });
//! }
//! ```

pub mod canvas;
pub mod image;
pub mod input;

#[doc(inline)]
pub use canvas::Canvas;
#[doc(inline)]
pub use image::{Color, Image};
