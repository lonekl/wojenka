pub mod object_conversion;
pub mod panels;
pub mod algorithms;
pub mod triangles;
pub mod values;

use std::time::{Duration, Instant};
use glium::{glutin, Display, Surface, VertexBuffer, Program, DrawParameters, Depth, DepthTest};
use glutin::{event_loop as glutin_event_loop, event as glutin_event};
use glutin_event_loop::EventLoop as GlutinEventLoop;
use glutin_event::{Event as GlutinEvent, WindowEvent};
use winit::event_loop::ControlFlow;
use war_economy_core::Game;
use crate::{ResultStringify, RuntimeSettings};
use crate::opengl::algorithms::{Camera, KeyControls};
use crate::opengl::object_conversion::map::map_tiles_to_vertexes;
use crate::opengl::panels::game::GamePanel;
use crate::opengl::panels::Panel;
use crate::units::{Angle, Matrix4x4, RotationXYZ};


pub struct OpenGlInterface {}

impl OpenGlInterface {

    pub fn new() -> Result<Self, String> {

        Ok(Self {})
    }


    pub fn run_game_loop(self, runtime_settings: RuntimeSettings) -> ! {
        let event_loop = glutin_event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new().with_title("War And Economy").with_maximized(true);
        let context_builder = glutin::ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        let mut panel: Box<dyn Panel> = Box::new(GamePanel::new(&display).unwrap());

        let mut last_frame_time = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            let cycle_start = Instant::now();
            let time_from_last_frame = cycle_start.duration_since(last_frame_time);
            *control_flow = ControlFlow::WaitUntil(cycle_start + runtime_settings.frame_length.checked_sub(time_from_last_frame).unwrap_or(Duration::ZERO));

            match event {
                GlutinEvent::WindowEvent { event: window_event, .. } => match window_event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, is_synthetic, .. } => panel.keyboard_event(input, is_synthetic).unwrap(),
                    _ => {},
                },
                GlutinEvent::RedrawEventsCleared => if time_from_last_frame >= runtime_settings.frame_length {
                    display.gl_window().window().request_redraw()
                },
                GlutinEvent::RedrawRequested(window_id) => {
                    if window_id != display.gl_window().window().id() {
                        panic!("Window id {window_id:?} doesn't match game window.");
                    }

                    panel.redraw(&display, last_frame_time.elapsed()).unwrap();
                    last_frame_time = Instant::now();

                },
                _ => {},
            }

            if time_from_last_frame >= runtime_settings.frame_length {
                //display.gl_window().window().request_redraw();
            }

        })
    }

}

const VERTEX_SHADER: &'static str = r#"
#version 150

in vec3 position;
in vec2 uv;

uniform mat4 projection;
uniform mat3 rotation;
uniform vec3 camera_position;

out vec2 v_uv;

void main() {

    gl_Position = projection * vec4(rotation * (position - camera_position), 1.0);

    v_uv = uv;

}

"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 150

in vec2 v_uv;

out vec4 color;

void main() {

    color = vec4(v_uv.x, 1.0, v_uv.y, 1.0);

}

"#;
