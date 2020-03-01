use packed_simd::*;
use pixel_canvas::{input::MouseState, prelude::*};
use rayon::prelude::*;
use std::ops::{Add, Mul};

const CHUNK: usize = 8;
type F32s = f32x8;
type I32s = i32x8;
type M32s = m32x8;

#[derive(Clone, Copy)]
struct C32 {
    r: F32s,
    i: F32s,
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
    fn len2(&self) -> F32s {
        self.r * self.r + self.i * self.i
    }

    fn diverges(&self) -> M32s {
        self.len2().gt(F32s::splat(4.0))
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
        let splay = F32s::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let coord = |x, y| C32 {
            r: (x - F32s::splat(half_width as f32)) / scale,
            i: (y - F32s::splat(half_height as f32)) / scale,
        };
        let c = coord(F32s::splat(mouse.x as f32), F32s::splat(mouse.y as f32));
        let width = image.width() as usize;
        image
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                let y = F32s::splat(y as f32);
                for (x, pix) in row.chunks_mut(CHUNK).enumerate() {
                    let x = F32s::splat((x * CHUNK) as f32) + splay;
                    let mut z = coord(x, y);
                    let mut i = I32s::splat(0);
                    let mut div = M32s::splat(false);
                    for _ in 0..127 {
                        z = z * z + c;
                        i += div.select(I32s::splat(0), I32s::splat(1));
                        div |= z.diverges();
                        if div.all() {
                            break;
                        }
                    }

                    for j in 0..CHUNK {
                        pix[j] = if div.extract(j) {
                            Color {
                                r: 255,
                                g: i.extract(j) as u8 * 2,
                                b: 0,
                            }
                        } else {
                            Color::BLACK
                        };
                    }
                }
            });
    });
}
