use std::fmt::{Debug, Display as FmtDisplay, Formatter, Write};
use glium::{Display as GliumDisplay, DrawError, ProgramCreationError, SwapBuffersError};
use glium::glutin::CreationError;
use glium::vertex::BufferCreationError as VertexBufferCreationError;



#[derive(Debug)]
pub enum GliumCreationError {

    Default(CreationError),
    Program(ProgramCreationError),
    VertexBuffer(VertexBufferCreationError),

}

#[derive(Debug)]
pub enum InterfaceError {

    WrongWindowId,
    GliumCreation(GliumCreationError),
    Draw(DrawError),
    BufferSwapping(SwapBuffersError),
    Other(String),

}

impl FmtDisplay for InterfaceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        fn output_error<E: FmtDisplay>(prefix: &str, formatter: &mut Formatter<'_>, error: E) -> std::fmt::Result {
            formatter.write_str(prefix)?;
            error.fmt(formatter)
        }

        formatter.write_str("PanelError > ")?;

        match self {
            InterfaceError::WrongWindowId => formatter.write_str("wrong window id."),
            InterfaceError::GliumCreation(creation) => match creation {
                GliumCreationError::Default(error) => output_error("data structure creation error > ", formatter, error),
                GliumCreationError::Program(error) => output_error("compilation failed > ", formatter, error),
                GliumCreationError::VertexBuffer(error) => output_error("sending of vertex buffer failed > ", formatter, error),
            },
            InterfaceError::Draw(error) => output_error("draw error > ", formatter, error),
            InterfaceError::BufferSwapping(error) => output_error("buffer swapping error > ", formatter, error),
            InterfaceError::Other(error) => formatter.write_str(error),
        }
    }
}



pub trait ToInterfaceError<T> {

    fn to_interface_error(self) -> Result<T, InterfaceError>;

}

impl<T> ToInterfaceError<T> for Result<T, CreationError> {
    fn to_interface_error(self) -> Result<T, InterfaceError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(InterfaceError::GliumCreation(GliumCreationError::Default(error))),
        }
    }
}

impl<T> ToInterfaceError<T> for Result<T, ProgramCreationError> {
    fn to_interface_error(self) -> Result<T, InterfaceError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(InterfaceError::GliumCreation(GliumCreationError::Program(error))),
        }
    }
}

impl<T> ToInterfaceError<T> for Result<T, VertexBufferCreationError> {
    fn to_interface_error(self) -> Result<T, InterfaceError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(InterfaceError::GliumCreation(GliumCreationError::VertexBuffer(error))),
        }
    }
}

impl<T> ToInterfaceError<T> for Result<T, DrawError> {
    fn to_interface_error(self) -> Result<T, InterfaceError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(InterfaceError::Draw(error)),
        }
    }
}

impl<T> ToInterfaceError<T> for Result<T, SwapBuffersError> {
    fn to_interface_error(self) -> Result<T, InterfaceError> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(InterfaceError::BufferSwapping(error)),
        }
    }
}
