use std::time::Duration;
use glium::{Display};
use winit::event::KeyboardInput;
use crate::opengl::error::InterfaceError;

pub mod game;


pub trait Panel {

    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), InterfaceError>;

    fn redraw(&mut self, display: &Display, last_frame_length: Duration) -> Result<(), InterfaceError>;

}
