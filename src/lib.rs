#![feature(specialization)]
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

type HandleFn<State> = fn(&CanvasInfo, &mut State, &Event<()>);

pub struct CanvasInfo {
    width: u32,
    height: u32,
    title: String,
    hidpi: bool,
    dpi: f64,
}

pub struct Canvas<State, Handler = HandleFn<State>> {
    info: CanvasInfo,
    image: Image,
    state: State,
    event_handler: Handler,
}

pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub physical_pixels: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self { x: 0, y: 0, physical_pixels: false }
    }

    pub fn physical() -> Self {
        Self { x: 0, y: 0, physical_pixels: true }
    }

    pub fn handle(info: &CanvasInfo, mouse: &mut MouseState, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let (x, y): (i32, i32) = (*position).into();
                mouse.x = x;
                mouse.y = info.height as i32 - y;
                if mouse.physical_pixels {
                    mouse.x = (mouse.x as f64 * info.dpi) as i32;
                    mouse.y = (mouse.y as f64 * info.dpi) as i32;
                }
            }
            _ => (),
        }
    }
}

impl Canvas<()> {
    pub fn new(width: u32, height: u32) -> Canvas<()> {
        Canvas {
            info: CanvasInfo {
                width,
                height,
                hidpi: true,
                dpi: 1.0,
                title: "Canvas".into(),
            },
            image: Image::new(width, height),
            state: (),
            event_handler: |_, (), _| {},
        }
    }
}

impl<State, Handler> Canvas<State, Handler>
where
    Handler: FnMut(&CanvasInfo, &mut State, &Event<()>) + 'static,
    State: 'static,
{
    pub fn state<NewState>(self, state: NewState) -> Canvas<NewState, HandleFn<NewState>> {
        Canvas {
            info: self.info,
            image: self.image,
            state,
            event_handler: |_, _, _| {},
        }
    }

    pub fn title(self, text: impl Into<String>) -> Self {
        Self {
            info: CanvasInfo {
                title: text.into(),
                ..self.info
            },
            ..self
        }
    }

    pub fn hidpi(self, enabled: bool) -> Self {
        Self {
            info: CanvasInfo {
                hidpi: enabled,
                ..self.info
            },
            ..self
        }
    }

    pub fn input<NewHandler>(self, callback: NewHandler) -> Canvas<State, NewHandler>
    where
        NewHandler: FnMut(&CanvasInfo, &mut State, &Event<()>) + 'static,
    {
        Canvas {
            info: self.info,
            image: self.image,
            state: self.state,
            event_handler: callback,
        }
    }

    pub fn render(mut self, mut callback: impl FnMut(&mut State, &mut Image) + 'static) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(&self.info.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                self.info.width as f64,
                self.info.height as f64,
            ))
            .with_resizable(false);
        let cb = glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        self.info.dpi = if self.info.hidpi {
            display.gl_window().window().hidpi_factor()
        } else {
            1.0
        };

        let width = (self.info.width as f64 * self.info.dpi) as u32;
        let height = (self.info.height as f64 * self.info.dpi) as u32;
        self.image = Image::new(width, height);

        let texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            width,
            height,
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
                            width: width as u32,
                            height: height as u32,
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
                event => (self.event_handler)(&self.info, &mut self.state, &event),
            }
        })
    }
}
