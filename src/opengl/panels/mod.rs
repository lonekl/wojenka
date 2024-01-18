use std::time::Duration;
use glium::Display;
use winit::event::KeyboardInput;

pub mod game;


pub trait Panel {

    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), String>;

    fn redraw(&mut self, display: &Display, last_frame_length: Duration) -> Result<(), String>;

}
