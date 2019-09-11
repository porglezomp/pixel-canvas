//! Types and utilities to represent colors.

use std::ops::{Add, Mul, Sub};

// @Todo: Explain colors.

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

impl Color {
    /// The color black.
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    /// The color white.
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    /// A convenience constructor for a color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

/// A trait to blend between two values by some factor.
pub trait Blend<T> {
    /// Blend between two values.
    /// ```rust
    /// # use pixel_canvas::prelude::*;
    /// // Blend entirely in integer math.
    /// assert_eq!(100.blend(200, 0), 100);
    /// assert_eq!(100.blend(200, 128), 150);
    /// assert_eq!(100.blend(200, 255), 200);
    /// // Blend with a floating point factor.
    /// assert_eq!(100.blend(200, 0.0), 100);
    /// assert_eq!(100.blend(200, 0.5), 150);
    /// assert_eq!(100.blend(200, 1.0), 200);
    /// ```
    fn blend(self, other: Self, factor: T) -> Self;
}

impl Blend<u8> for u8 {
    fn blend(self, other: u8, factor: u8) -> u8 {
        (self as i16 + ((other as i16 - self as i16) * (factor as i16) / 255)) as u8
    }
}

impl Blend<u8> for Color {
    fn blend(self, other: Color, factor: u8) -> Color {
        Color {
            r: self.r.blend(other.r, factor),
            g: self.g.blend(other.g, factor),
            b: self.b.blend(other.b, factor),
        }
    }
}

impl Blend<f32> for u8 {
    fn blend(self, other: u8, factor: f32) -> u8 {
        (self as f32 * (1.0 - factor) + other as f32 * factor) as u8
    }
}

impl Blend<f32> for Color {
    fn blend(self, other: Color, factor: f32) -> Color {
        Color {
            r: self.r.blend(other.r, factor),
            g: self.g.blend(other.g, factor),
            b: self.b.blend(other.b, factor),
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: ((self.r as u16 * rhs.r as u16) >> 8) as u8,
            g: ((self.g as u16 * rhs.g as u16) >> 8) as u8,
            b: ((self.b as u16 * rhs.b as u16) >> 8) as u8,
        }
    }
}

impl Mul<u8> for Color {
    type Output = Color;
    fn mul(self, rhs: u8) -> Color {
        Color::BLACK.blend(self, rhs)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color::BLACK.blend(self, rhs)
    }
}
