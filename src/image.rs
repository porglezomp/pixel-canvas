//! The [`Image`] is what you manipulate to produce your art.
//!
//! Every frame you are given a mutable reference to the existing frame, and
//! are able to modify it to produce your image.
//!
//! [`Image`]: struct.Image.html

// @Todo: Add multiple pixel formats?
// @Todo: Seaparate stride from width, and document.

use crate::color::Color;
use glium::texture::{ClientFormat, RawImage2d, Texture2dDataSource};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// An image for editing.
///
/// It dereferences to a slice of [`Color`], so you can directly manipulate
/// pixels via regular (mutable) slice methods. In addition, you can index
/// into the image by `(row, column)` pairs.
///
/// [`Color`]: ../color/struct.Color.html
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

/// A row/column pair for indexing into an image.
/// Distinct from an x/y pair.
pub struct RC(pub usize, pub usize);

/// An x/y pair for indexing into an image.
/// Distinct from a row/column pair.
pub struct XY(pub usize, pub usize);

impl Image {
    /// The width of the image in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of the image in pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Create an all-black image with the given dimensions.
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color { r: 0, g: 0, b: 0 }; (width * height) as usize],
        }
    }

    /// Fill the image with a single solid color.
    pub fn fill(&mut self, color: Color) {
        for pix in &mut self.pixels {
            *pix = color;
        }
    }
}

impl Index<RC> for Image {
    type Output = Color;
    fn index(&self, RC(row, col): RC) -> &Self::Output {
        &self.pixels[(row * self.width + col) as usize]
    }
}

impl IndexMut<RC> for Image {
    fn index_mut(&mut self, RC(row, col): RC) -> &mut Self::Output {
        &mut self.pixels[(row * self.width + col) as usize]
    }
}

impl Index<XY> for Image {
    type Output = Color;
    fn index(&self, XY(x, y): XY) -> &Self::Output {
        &self.pixels[(y * self.width + x) as usize]
    }
}

impl IndexMut<XY> for Image {
    fn index_mut(&mut self, XY(x, y): XY) -> &mut Self::Output {
        &mut self.pixels[(y * self.width + x) as usize]
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
