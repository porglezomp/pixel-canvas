use crate::image::Image;
use glium::{
    glutin::{
        self,
        event::{Event, StartCause},
        event_loop::ControlFlow,
    },
    Rect, Surface,
};
use std::time::{Duration, Instant};

type HandleFn<State> = fn(&CanvasInfo, &mut State, &Event<()>);

pub struct CanvasInfo {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub hidpi: bool,
    pub dpi: f64,
    pub show_ms: bool,
}

pub struct Canvas<State, Handler = HandleFn<State>> {
    info: CanvasInfo,
    image: Image,
    state: State,
    event_handler: Handler,
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
                show_ms: false,
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

    pub fn show_ms(self, enabled: bool) -> Self {
        Self {
            info: CanvasInfo {
                show_ms: enabled,
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
                    next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
                    let frame_start = Instant::now();

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

                    let frame_end = Instant::now();
                    if self.info.show_ms {
                        display.gl_window().window().set_title(&format!(
                            "{} - {:3}ms",
                            self.info.title,
                            frame_end.duration_since(frame_start).as_millis()
                        ));
                    }
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
