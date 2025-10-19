pub(crate) use crate::traits::Resize;

mod glfw_window;

use crate::{traits::Updates, types::Size2D, window::glfw_window::GLFWWindow};

pub fn new(params: WindowParams) -> WindowInitResult<Box<impl WindowLike>> {
    match GLFWWindow::from_params(params) {
        Ok(w) => Ok(Box::new(w)),
        Err(_e) => Err(WindowInitError {}),
    }
}

pub trait WindowLike: Resize<Size2D> + Updates {
    fn present_frame(&mut self);
}

pub struct WindowParams {
    pub width: u32,
    pub height: u32,
    pub handles_input: bool,
}

type WindowInitResult<T> = std::result::Result<T, WindowInitError>;

#[derive(Debug, Clone)]
pub struct WindowInitError {}
