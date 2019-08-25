//! The [`Image`] is what you manipulate to produce your art.
//!
//! Every frame you are given a mutable reference to the existing frame, and
//! are able to modify it to produce your image.
//!
//! [`Image`]: struct.Image.html

// @Todo: Add multiple pixel formats?
// @Todo: Seaparate stride from width, and document.
// @Todo: Explain colors.

use glium::texture::{ClientFormat, RawImage2d, Texture2dDataSource};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// A single RGB-888 color.
// This must be repr(C) in order to directly upload to the GPU.
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Color {
    /// The red component.
    pub r: u8,
    /// The green component.
    pub g: u8,
    /// The blue component.
    pub b: u8,
}

/// An image for editing.
///
/// It dereferences to a slice of [`Color`], so you can directly manipulate
/// pixels via regular (mutable) slice methods. In addition, you can index
/// into the image by `(row, column)` pairs.
///
/// [`Color`]: struct.Color.html
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Image {
    /// The width of the image in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the image in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Create an all-black image with the given dimensions.
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color { r: 0, g: 0, b: 0 }; (width * height) as usize],
        }
    }
}

impl Index<(u32, u32)> for Image {
    type Output = Color;
    fn index(&self, (row, col): (u32, u32)) -> &Self::Output {
        &self.pixels[(row * self.width + col) as usize]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (row, col): (u32, u32)) -> &mut Self::Output {
        &mut self.pixels[(row * self.width + col) as usize]
    }
}

impl Deref for Image {
    type Target = [Color];
    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<'a> Texture2dDataSource<'a> for &'a Image {
    type Data = u8;
    fn into_raw(self) -> RawImage2d<'a, Self::Data> {
        RawImage2d {
            data: Cow::Borrowed(unsafe {
                std::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, self.pixels.len() * 3)
            }),
            width: self.width as u32,
            height: self.height as u32,
            format: ClientFormat::U8U8U8,
        }
    }
}
