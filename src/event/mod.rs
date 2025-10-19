pub mod window;

use glfw::{ClientApiHint, Context, Glfw, WindowHint};
use window::Window;

use crate::event::window::WindowParams;

#[derive(Debug)]
pub struct EventContext {
    glfw: Glfw,

    windows: Vec<Window>,
}

impl EventContext {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

        Self {
            glfw,
            windows: vec![],
        }
    }

    pub fn get_window(&mut self, index: usize) -> Option<&mut Window> {
        self.windows.get_mut(index)
    }

    pub fn create_window(&mut self, params: WindowParams) -> &mut Window {
        let new_window = Window::new(&mut self.glfw, params);

        self.windows.push(new_window);

        self.windows.last_mut().unwrap()
    }

    // TODO: Make this return a result
    pub fn update(&mut self) {
        self.glfw.poll_events();

        for window in &mut self.windows {
            for (_, event) in glfw::flush_messages(window.events_mut()) {
                match event {
                    glfw::WindowEvent::Close => panic!("Window closed."),
                    glfw::WindowEvent::Pos(_, _)
                    | glfw::WindowEvent::Size(_, _)
                    | glfw::WindowEvent::Refresh
                    | glfw::WindowEvent::Focus(_)
                    | glfw::WindowEvent::Iconify(_)
                    | glfw::WindowEvent::FramebufferSize(_, _)
                    | glfw::WindowEvent::MouseButton(_, _, _)
                    | glfw::WindowEvent::CursorPos(_, _)
                    | glfw::WindowEvent::CursorEnter(_)
                    | glfw::WindowEvent::Scroll(_, _)
                    | glfw::WindowEvent::Key(_, _, _, _)
                    | glfw::WindowEvent::Char(_)
                    | glfw::WindowEvent::CharModifiers(_, _)
                    | glfw::WindowEvent::FileDrop(_)
                    | glfw::WindowEvent::Maximize(_)
                    | glfw::WindowEvent::ContentScale(_, _) => {}
                }
            }
        }
    }
}

impl Default for EventContext {
    fn default() -> Self {
        Self::new()
    }
}
