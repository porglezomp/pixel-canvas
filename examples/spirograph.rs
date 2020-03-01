use pixel_canvas::{input::MouseState, prelude::*};

fn spirograph(l: f32, k: f32, t: f32) -> (f32, f32) {
    const R: f32 = 0.9;
    let k1 = 1.0 - k;
    let k2 = k1 * t / k.max(0.001);
    (
        R * (k1 * t.cos() + l * k * k2.cos()),
        R * (k1 * t.sin() - l * k * k2.sin()),
    )
}

fn main() {
    let canvas = Canvas::new(1280, 720)
        .title("Sburb")
        .hidpi(true)
        .state(MouseState::new())
        .input(MouseState::handle_input);
    canvas.render(|mouse, image| {
        image.fill(Color::BLACK);
        let aspect = (image.height() as f32).min(image.width() as f32);
        let l = (mouse.x / 15 * 15) as f32 / image.width() as f32;
        let k = (mouse.y / 15 * 15) as f32 / image.height() as f32;
        for t in 0..100000 {
            let (x, y) = spirograph(l, k, t as f32 / 100.0);
            let x = (x * aspect / 2.0) as usize + image.width() / 2;
            let y = (y * aspect / 2.0) as usize + image.height() / 2;
            image[XY(x, y)] = Color::rgb(127, 255, 0);
        }
    });
}
