use std::time::Duration;
use glium::{Display};
use winit::event::{KeyboardInput, MouseScrollDelta, TouchPhase};
use crate::opengl::error::InterfaceError;

pub mod game;



pub trait Panel {

    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), InterfaceError>;
    fn mouse_wheel_event(&mut self, delta: MouseScrollDelta, phase: TouchPhase) -> Result<(), InterfaceError>;

    fn redraw(&mut self, display: &Display, last_frame_length: Duration) -> Result<(), InterfaceError>;

}
