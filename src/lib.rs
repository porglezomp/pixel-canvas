use glium::{
    glutin::{
        event::{Event, StartCause, WindowEvent},
        event_loop::ControlFlow,
    },
    texture::{ClientFormat, RawImage2d, Texture2dDataSource},
    Rect, Surface,
};
use std::{
    borrow::Cow,
    ops::{Index, IndexMut},
    time::{Duration, Instant},
};

pub use glium::glutin;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Image {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

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

pub struct Canvas<State> {
    width: u32,
    height: u32,
    title: String,
    image: Image,
    state: State,
    event_handler: Option<Box<dyn FnMut(&Image, &mut State, &Event<()>)>>,
}

#[derive(Default)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
}

impl MouseState {
    pub fn handle(image: &Image, mouse: &mut MouseState, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let (x, y): (i32, i32) = (*position).into();
                mouse.x = x;
                mouse.y = image.height() as i32 - y;
            }
            _ => (),
        }
    }
}

impl<State> Canvas<State>
where
    State: 'static,
{
    pub fn new(width: u32, height: u32, state: State) -> Canvas<State> {
        Canvas {
            width,
            height,
            title: "Canvas".into(),
            image: Image::new(width, height),
            state,
            event_handler: None,
        }
    }

    pub fn title(self, text: impl Into<String>) -> Self {
        Self {
            title: text.into(),
            ..self
        }
    }

    pub fn handle_events(
        self,
        callback: impl FnMut(&Image, &mut State, &Event<()>) + 'static,
    ) -> Self {
        Self {
            event_handler: Some(Box::new(callback)),
            ..self
        }
    }

    pub fn render(mut self, mut callback: impl FnMut(&mut State, &mut Image) + 'static) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                self.width as f64,
                self.height as f64,
            ))
            .with_resizable(false);
        let cb = glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            self.width as u32,
            self.height as u32,
        )
        .unwrap();

        let mut next_frame_time = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            match event {
                Event::NewEvents(StartCause::ResumeTimeReached { .. })
                | Event::NewEvents(StartCause::Init) => {
                    callback(&mut self.state, &mut self.image);
                    texture.write(
                        Rect {
                            left: 0,
                            bottom: 0,
                            width: self.width as u32,
                            height: self.height as u32,
                        },
                        &self.image,
                    );

                    let target = display.draw();
                    texture
                        .as_surface()
                        .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
                    target.finish().unwrap();

                    next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
                }
                glutin::event::Event::WindowEvent {
                    event: glutin::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                event => {
                    if let Some(handler) = &mut self.event_handler {
                        handler(&self.image, &mut self.state, &event)
                    }
                }
            }
        })
    }
}
