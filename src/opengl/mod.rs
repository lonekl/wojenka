pub mod object_conversion;
pub mod panels;
pub mod algorithms;
pub mod error;
pub mod triangles;
pub mod values;

use std::time::{Duration, Instant};
use glium::{glutin, Display};
use glutin::{event_loop as glutin_event_loop, event as glutin_event};
use glutin_event::{Event as GlutinEvent, WindowEvent};
use winit::event_loop::ControlFlow;
use crate::{RuntimeSettings};
use crate::opengl::error::InterfaceError;
use crate::opengl::panels::game::GamePanel;
use crate::opengl::panels::Panel;
use crate::util::{GlobalLogger, PoisonClearer, ResultLoggerExcept};



pub struct OpenGlInterface {}

impl OpenGlInterface {

    pub fn new() -> Result<Self, String> {

        Ok(Self {})
    }


    pub fn run_game_loop(self, logger: GlobalLogger, runtime_settings: RuntimeSettings) -> ! {
        let event_loop = glutin_event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new().with_title("Wojenka").with_maximized(true);
        let context_builder = glutin::ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, &event_loop).expect_logger(&logger, "Failed to create window");

        let mut panel: Box<dyn Panel> = Box::new(GamePanel::new(&display).expect_logger(&logger, "Failed to create game panel"));

        let mut last_frame_time = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            let cycle_start = Instant::now();
            let time_from_last_frame = cycle_start.duration_since(last_frame_time);
            *control_flow = ControlFlow::WaitUntil(
                cycle_start
                    + runtime_settings.frame_length
                    .checked_sub(time_from_last_frame)
                    .unwrap_or(Duration::ZERO)
            );

            let event_pass_result: Result<(), InterfaceError> = try {

                match event {
                    GlutinEvent::WindowEvent { event: window_event, .. } => match window_event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, is_synthetic, .. } => panel.keyboard_event(input, is_synthetic)?,
                        WindowEvent::MouseWheel { delta, phase, .. } => panel.mouse_wheel_event(delta, phase)?,
                        _ => {},
                    },
                    GlutinEvent::RedrawEventsCleared => if time_from_last_frame >= runtime_settings.frame_length {
                        display.gl_window().window().request_redraw()
                    },
                    GlutinEvent::RedrawRequested(window_id) => {
                        if window_id != display.gl_window().window().id() {
                            Err(InterfaceError::WrongWindowId)?;
                        }

                        panel.redraw(&display, last_frame_time.elapsed())?;
                        last_frame_time = Instant::now();

                    },
                    _ => {},
                }

            };

            match event_pass_result {
                Ok(_) => {},
                Err(error) => logger.lock().ignore_poison().err(&format!("Event loop error caught: {error}.")),
            }

        })
    }

}
