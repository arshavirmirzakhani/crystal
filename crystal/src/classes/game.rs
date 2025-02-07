use crate::{classes::conf::GameConfig, window::window::GameWindow};
use winit::event_loop::{ControlFlow, EventLoop};

// initialize the game object (basically loading the configuration)
trait Init {
    fn init(&mut self);
}

trait Run {
    fn run(&mut self);
}

// the Game class manages and handles anything in the game like scenes, objects, rendering, physics, Input/Output, etc
pub struct Game {
    pub conf: GameConfig,
    pub game_window: GameWindow,
}

impl Run for Game {
    fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        let result = event_loop.run_app(&mut self.game_window);

        assert!(result.is_err());
    }
}

impl Init for Game {
    fn init(&mut self) {
        println!("{:?}", self.conf.game_name);
    }
}
