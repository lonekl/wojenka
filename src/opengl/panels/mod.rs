use std::time::Duration;
use std::fmt::{Debug, Display as FmtDisplay, Formatter, Write};
use glium::{Display as GliumDisplay, DrawError, ProgramCreationError};
use glium::glutin::CreationError;
use winit::event::KeyboardInput;

pub mod game;


pub trait Panel {

    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), PanelError>;

    fn redraw(&mut self, display: &GliumDisplay, last_frame_length: Duration) -> Result<(), PanelError>;

}




pub enum PanelError {

    WrongWindowId,
    CreationError(CreationError),
    ProgramCreationError(ProgramCreationError),
    DrawError(DrawError),
    Other(String),

}

impl FmtDisplay for PanelError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        formatter.write_str("PanelError > ")?;

        match self {
            PanelError::WrongWindowId => formatter.write_str("wrong window id."),
            PanelError::CreationError(error) => {
                formatter.write_str("creation error > ")?;
                error.fmt(formatter)?;
                formatter.write_str(".")
            },
            PanelError::ProgramCreationError(error) => {
                formatter.write_str("program creation error > ")?;
                error.fmt(formatter)?;
                formatter.write_str(".")
            },
            PanelError::DrawError(error) => {
                formatter.write_str("draw error > ")?;
                error.fmt(formatter)?;
                formatter.write_str(".")
            },
            PanelError::Other(error) => formatter.write_str(error),
        }?;

        Ok(())
    }
}



pub trait ToPanelError<T> {

    fn to_panel_error(self) -> Result<T, PanelError>;

}

impl<T> ToPanelError<T> for Result<T, DrawError> {
    fn to_panel_error(self) -> Result<T, PanelError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(PanelError::DrawError(error)),
        }
    }
}

impl<T> ToPanelError<T> for Result<T, CreationError> {
    fn to_panel_error(self) -> Result<T, PanelError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(PanelError::CreationError(error)),
        }
    }
}

impl<T> ToPanelError<T> for Result<T, ProgramCreationError> {
    fn to_panel_error(self) -> Result<T, PanelError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(PanelError::ProgramCreationError(error)),
        }
    }
}
