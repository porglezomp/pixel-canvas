use pixel_canvas::prelude::*;
use rand;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

#[derive(Debug)]
struct Camera {
    pos: Vec3,
    dir: Vec3,
}

#[derive(Debug)]
struct Hit {
    point: Vec3,
    normal: Vec3,
}

fn render(pos: Vec3, dir: Vec3) -> Color {
    const SHADOW_SAMPLES: usize = 10;
    let mut rng = rand::thread_rng();
    let shadow_dist = Normal::new(0.0, 0.02).unwrap();
    let upness = dir.dot(xyz(0.0, 0.0, 1.0));
    let sky = rgb(255, 220, 200).blend(rgb(64, 127, 255), upness.restrict(0.0..=1.0));
    let light_dir = xyz(2.0, 0.1, 1.5).normal();
    match march(pos, dir, 300, 0.5) {
        Some(hit) => {
            let dist = (hit.point - pos).len();
            if dist > 150.0 {
                return sky;
            }
            let fog = dist / 150.0;
            let fog = fog * fog;
            let h = (hit.point.z + 2.0) * 30.0;
            let r = h.restrict(0.0..=180.0) as u8;
            let g = (h * 2.0).restrict(0.0..=200.0) as u8;
            let albedo = rgb(r, g, 64);
            let sky_light = rgb(0, 64, 128)
                * hit
                    .normal
                    .dot(xyz(0.0, 0.0, 1.0))
                    .remap(0.0..1.0, 0.3..1.0)
                    .restrict(0.0..=1.0);
            let sun_factor: f32 = (0..SHADOW_SAMPLES)
                .map(|_| {
                    let mut rand = || shadow_dist.sample(&mut rng);
                    let penum = xyz(rand(), rand(), rand());
                    if march(hit.point + hit.normal * 0.04, light_dir + penum, 100, 0.5).is_none() {
                        1.0 / SHADOW_SAMPLES as f32
                    } else {
                        0.0
                    }
                })
                .sum();
            let sun_light =
                rgb(255, 240, 220) * light_dir.dot(hit.normal).restrict(0.0..=1.0) * sun_factor;
            let light = sun_light + sky_light;
            (albedo * light).blend(sky, fog)
        }
        None => sky,
    }
}

fn height(x: f32, y: f32) -> f32 {
    let ground = (x * 0.5 + (y * 0.1).sin()).sin() * (y * 0.5 + (x * 0.05).cos()).sin()
        + (x * 0.1).sin() * (y * 0.1).sin() * 2.0
        + (x * 0.02).sin() * (y * 0.02).sin() * 4.0;
    (ground * ground.abs()).restrict(-1.0..)
}

fn normal(x: f32, y: f32) -> Vec3 {
    let eps = 0.001;
    let root = xyz(x, y, height(x, y));
    let a = xyz(x + eps, y, height(x + eps, y));
    let b = xyz(x, y + eps, height(x, y + eps));
    (a - root).cross(b - root).normal()
}

fn march(mut pos: Vec3, dir: Vec3, steps: usize, dt: f32) -> Option<Hit> {
    let dir = dir.normal();
    let mut last_pos = pos;
    for _ in 0..steps {
        let height = height(pos.x, pos.y);
        if pos.z < height {
            if dt > 0.005 {
                let new_step = dt / 2.0;
                return march(last_pos, dir, 64, new_step);
            }
            return Some(Hit {
                point: pos,
                normal: normal(pos.x, pos.y),
            });
        }
        let len = ((pos.z - height) / 2.0).restrict(dt..);
        last_pos = pos;
        pos = pos + dir * len;
    }
    None
}

fn main() {
    let canvas = Canvas::new(300, 720)
        .hidpi(false)
        .title("Mountains")
        .state(Camera {
            pos: xyz(0.0, 0.0, 5.0),
            dir: xyz(0.0, 1.0, 0.0),
        })
        .render_on_change(true);

    let dist = Normal::new(0.0, 0.001).unwrap();
    canvas.render(move |camera, img| {
        let (w, h) = (img.width() as usize, img.height() as usize);
        let aspect = w as f32 / h as f32;
        img.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let mut rng = rand::thread_rng();
            let y = (y as f32).remap(0.0..h as f32, -1.0..1.0);
            for (x, pixel) in row.iter_mut().enumerate() {
                let x = (x as f32).remap(0.0..w as f32, -1.0..1.0) * aspect;
                let (x, y) = (x + dist.sample(&mut rng), y + dist.sample(&mut rng));
                let dir = camera.dir(x, y);
                *pixel = render(camera.pos, dir);
            }
        });
    });
}

impl Camera {
    fn dir(&self, x: f32, y: f32) -> Vec3 {
        Vec3 { x, y: 1.0, z: y }.normal()
    }
}

fn xyz(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::xyz(x, y, z)
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r, g, b)
}
