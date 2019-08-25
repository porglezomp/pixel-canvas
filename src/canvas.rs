//! The `Canvas` is the main entry point of the library. It handles window
//! creation and input, calls your render callback, and presents the image on
//! the screen.
//!
//! You create and configure a `Canvas` via builder methods. You can create
//! a perfectly functionl, bare-bones canvas just by calling `Canvas::new`
//! with your dimensions, and then calling `render`. If you
//! want a fancier canvas (like handling input, with a custom title, etc.) you
//! can configure that as well. For example:
//! ```rust
//! # use pixel_canvas::{Canvas, input::MouseState};
//! let canvas = Canvas::new(512, 512)
//!     .title("Tile")
//!     .hidpi(true)
//!     .show_ms(true)
//!     .state(MouseState::new())
//!     .input(MouseState::handle_input);
//! ```
//! This adds a 512x512 window called "Title", that renders in hidpi mode,
//! displays the frame rendering time, and tracks the position of the mouse in
//! physical pixels. For more information on event handlers, see the `input`
//! module.
//!
//! Once you've created your canvas, you can use it to render your art. Do
//! whatever you want in the render callback, the image you build will be
//! displayed in the window when your render callback returns.
//! ```rust,no_run
//! # use pixel_canvas::{Canvas, Color, input::MouseState};
//! # fn make_your_own_color(x: usize, y: usize, mx: i32, my: i32) -> Color {
//! #     Color { r: 0, g: 0, b: 0 }
//! # }
//! # let canvas = Canvas::new(512, 512).state(MouseState::new());
//! canvas.render(|mouse, image| {
//!     let width = image.width() as usize;
//!     for (y, row) in image.chunks_mut(width).enumerate() {
//!         for (x, pixel) in row.iter_mut().enumerate() {
//!             *pixel = make_your_own_color(x, y, mouse.x, mouse.y);
//!         }
//!     }
//! });
//! ```

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

type EventHandler<State> = fn(&CanvasInfo, &mut State, &Event<()>);

/// Information about the `Canvas`.
pub struct CanvasInfo {
    /// The width of the canvas, in virtual pixels.
    pub width: u32,
    /// The height of the canvas, in vritual pixels.
    pub height: u32,
    /// The base title for the window.
    pub title: String,
    /// Whether the canvas will render in hidpi mode.
    pub hidpi: bool,
    /// The DPI factor. If hidpi is on, the virtual dimensions are multiplied
    /// by this factor to create the actual image resolution. For example, if
    /// you're on a Retina Macbook, this will be 2.0, so the image will be
    /// twice the resolution that you specified.
    pub dpi: f64,
    /// Whether the window title will display the time to render a frame.
    pub show_ms: bool,
}

/// A `Canvas` manages a window and event loop, handing the current state to
/// the renderer, and presenting its image on the screen.
pub struct Canvas<State, Handler = EventHandler<State>> {
    info: CanvasInfo,
    image: Image,
    state: State,
    event_handler: Handler,
}

impl Canvas<()> {
    /// Create a new canvas with a given virtual window dimensions.
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
    /// Set the attached state.
    ///
    /// Attaching a new state object will reset the input handler.
    pub fn state<NewState>(self, state: NewState) -> Canvas<NewState, EventHandler<NewState>> {
        Canvas {
            info: self.info,
            image: self.image,
            state,
            event_handler: |_, _, _| {},
        }
    }

    /// Set the title on the canvas window.
    pub fn title(self, text: impl Into<String>) -> Self {
        Self {
            info: CanvasInfo {
                title: text.into(),
                ..self.info
            },
            ..self
        }
    }

    /// Toggle hidpi render.
    ///
    /// Defaults to true.
    /// If you have a hidpi monitor, this will cause the image to be larger
    /// than the dimensions you specified when creating the canvas.
    pub fn hidpi(self, enabled: bool) -> Self {
        Self {
            info: CanvasInfo {
                hidpi: enabled,
                ..self.info
            },
            ..self
        }
    }

    /// Whether to show a frame duration in the title bar.
    ///
    /// Defaults to false.
    pub fn show_ms(self, enabled: bool) -> Self {
        Self {
            info: CanvasInfo {
                show_ms: enabled,
                ..self.info
            },
            ..self
        }
    }

    /// Attach an input handler.
    ///
    /// Your input handler must be compatible with any state that you've set
    /// previously. Your event handler will be called for each event with the
    /// canvas information, the current state, and the inciting event.
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

    /// Provide a rendering callback.
    ///
    /// The canvas will call your rendering callback on demant, with the
    /// current state and a reference to the image. Currently this will be
    /// called at 60 fps.
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