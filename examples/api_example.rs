use pixel_canvas::{input::MouseState, Canvas, Color};

fn main() {
    let mut t = 0;
    let canvas = Canvas::new(1024, 512)
        .title("Rings")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    canvas.render(move |mouse, image| {
        for row in 0..image.height() {
            for col in 0..image.width() {
                let dx = col as i32 - mouse.x;
                let dy = row as i32 - mouse.y;
                let dist = dx * dx + dy * dy;
                let r = if dist < t * t { 255 } else { 0 };
                let g = ((r as u16).wrapping_mul(t as u16) >> 8) as u8;
                let b = ((g as u32).wrapping_mul(dist as u32) >> 12) as u8;
                image[(row, col)] = Color { r, g, b };
            }
        }
        t += 1;
    });
}
