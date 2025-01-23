use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::classes::conf::GameConfig;
trait LoadConf {
    fn load_conf(&mut self, conf: GameConfig);
}

pub struct GameWindow {
    pub window: Option<Window>,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl LoadConf for GameWindow {
    fn load_conf(&mut self, conf: GameConfig) {
        self.window_title = conf.window_title;
        self.window_width = conf.window_width;
        self.window_height = conf.window_height;
    }
}

impl ApplicationHandler for GameWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
