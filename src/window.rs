use glfw::*;

pub struct Window {
    pub glfw: glfw::Glfw,
    pub ptr: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
    
        window.make_current();
        window.set_all_polling(true);

        Window { glfw, ptr: window, events }
    }
    pub fn swap_buffers(&mut self) {
        self.ptr.swap_buffers();
    }
}
