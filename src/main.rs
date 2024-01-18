#[macro_use]
pub extern crate glium;

use std::fmt::Debug;
use std::time::Duration;

pub mod opengl;
pub mod units;



fn main() -> Result<(), String> {
    let runtime_settings = RuntimeSettings::new();
    let opengl = opengl::OpenGlInterface::new()?;

    opengl.run_game_loop(runtime_settings)
}



pub struct RuntimeSettings {

    frame_length: Duration,

}

impl RuntimeSettings {

    pub fn new() -> Self {

        Self {
            frame_length: Duration::from_secs_f64(1.0 / 60.0),
        }
    }

}



pub trait ResultStringify<T> {

    fn stringify_error(self) -> Result<T, String>;

}

impl<T, E: Debug> ResultStringify<T> for Result<T, E> {
    fn stringify_error(self) -> Result<T, String> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err( format!("{error:?}"))
        }
    }
}
