use glfw::{ClientApiHint, Context, WindowHint};

use crate::{
    types::Size2D,
    window::{Resize, WindowLike, WindowParams},
};

pub(super) struct GLFWWindow {
    size: Size2D,
    // TODO: Move this handle into a static member of window or whatever the rust equivalent of
    // that is
    glfw_handle: glfw::Glfw,

    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl GLFWWindow {
    pub fn from_params(params: WindowParams) -> Result<Self, glfw::InitError> {
        let mut glfw = glfw::init(glfw::fail_on_errors)?;

        // glfw.window_hint(WindowHint::ClientApi(ClientApiHint::OpenGl));

        let (mut window, events) = glfw
            .create_window(
                params.width,
                params.height,
                "Hello this is window",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        while !window.should_close() {
            window.swap_buffers();
            glfw.poll_events();
            for (_, _event) in glfw::flush_messages(&events) {}
        }

        Ok(GLFWWindow {
            size: Size2D {
                width: params.width,
                height: params.height,
            },
            glfw_handle: glfw,
            window,
            events,
        })
    }
}

impl Resize for GLFWWindow {
    fn set_size<S: Into<Size2D>>(&mut self, new_size: Size2D) {
        self.size = new_size;
    }

    fn get_size(&self) -> Size2D {
        return self.size;
    }
}

impl WindowLike for GLFWWindow {}
