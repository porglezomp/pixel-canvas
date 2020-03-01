use pixel_canvas::{input::MouseState, prelude::*};
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
struct C32 {
    r: f32,
    i: f32,
}

impl Add for C32 {
    type Output = C32;
    fn add(self, rhs: C32) -> C32 {
        C32 {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl Mul for C32 {
    type Output = C32;
    fn mul(self, rhs: C32) -> C32 {
        C32 {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl C32 {
    fn len2(&self) -> f32 {
        self.r * self.r + self.i * self.i
    }

    fn diverges(&self) -> bool {
        self.len2() > 4.0
    }
}

fn main() {
    let canvas = Canvas::new(1280, 720)
        .title("Julia Set")
        .hidpi(true)
        .show_ms(true)
        .state(MouseState::new())
        .input(MouseState::handle_input);
    canvas.render(|mouse, image| {
        let half_width = image.width() as i32 / 2;
        let half_height = image.height() as i32 / 2;
        let scale = half_height as f32 / 1.2;
        let coord = |x, y| C32 {
            r: (x - half_width) as f32 / scale,
            i: (y - half_height) as f32 / scale,
        };
        let c = coord(mouse.x, mouse.y);
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pix) in row.iter_mut().enumerate() {
                let mut z = coord(x as i32, y as i32);
                let mut i = 0;
                for _ in 0..127 {
                    z = z * z + c;
                    i += 1;
                    if z.diverges() {
                        break;
                    }
                }
                *pix = if z.diverges() {
                    Color {
                        r: 255,
                        g: i * 2,
                        b: 0,
                    }
                } else {
                    Color::BLACK
                };
            }
        }
    });
}
