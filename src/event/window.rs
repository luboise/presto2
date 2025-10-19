use glfw::{Context, Glfw, GlfwReceiver, WindowEvent};

#[derive(Debug, Clone)]
pub struct WindowParams {
    pub width: u32,
    pub height: u32,
    pub handles_input: bool,
}

impl Default for WindowParams {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            handles_input: true,
        }
    }
}

#[derive(Debug)]
pub struct Window {
    handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub(crate) fn new(glfw: &mut Glfw, params: WindowParams) -> Self {
        let (mut window_handle, events) = glfw
            .create_window(
                params.width,
                params.height,
                "Unnamed window",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window_handle.set_key_polling(false);

        Self {
            handle: window_handle,
            events,
        }
    }

    /* For OpenGL Windows Only
    pub fn focus(&mut self) {
        self.handle.make_current();
    }
    */

    pub(crate) fn events_mut(&mut self) -> &mut GlfwReceiver<(f64, WindowEvent)> {
        &mut self.events
    }

    pub(crate) fn handle(&self) -> &glfw::PWindow {
        &self.handle
    }

    pub(crate) fn handle_mut(&mut self) -> &mut glfw::PWindow {
        &mut self.handle
    }
}
