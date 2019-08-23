use pixel_canvas::{Canvas, Color, MouseState};

fn add(z1: (f32, f32), z2: (f32, f32)) -> (f32, f32) {
    (z1.0 + z2.0, z1.1 + z2.1)
}

fn mul(z1: (f32, f32), z2: (f32, f32)) -> (f32, f32) {
    (z1.0 * z2.0 - z1.1 * z2.1, z1.0 * z2.1 + z1.1 * z2.0)
}

fn dist2(z: (f32, f32)) -> f32 {
    z.0 * z.0 + z.1 * z.1
}

fn step(c: (f32, f32), z: (f32, f32)) -> (f32, f32) {
    add(mul(z, z), c)
}

fn main() {
    let canvas = Canvas::new(1280, 720)
        .title("Julia Set")
        .show_ms(true)
        .state(MouseState::physical())
        .input(MouseState::handle);
    canvas.render(|mouse, image| {
        let half_width = image.width() as i32 / 2;
        let half_height = image.height() as i32 / 2;
        let scale = half_height as f32 / 1.2;
        let coord = |x, y| {
            (
                (x - half_width) as f32 / scale,
                (y - half_height) as f32 / scale,
            )
        };
        let c = coord(mouse.x, mouse.y);
        for row in 0..image.height() {
            for col in 0..image.width() {
                let mut z = coord(col as i32, row as i32);
                let mut g = 0;
                let mut r = 255;
                for _ in 0..28 {
                    z = step(c, z);
                    g += 9;
                    if dist2(z) > 4.0 {
                        break;
                    }
                }
                if dist2(z) < 4.0 {
                    r = 0;
                    g = 0;
                }
                image[(row, col)] = Color { r, g, b: 0 };
            }
        }
    });
}
