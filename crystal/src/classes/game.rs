use crate::classes::conf::GameConfig;

trait Init {
    fn init(&mut self);
}

pub struct Game {
    pub conf: GameConfig,
}

impl Init for Game {
    fn init(&mut self) {
        println!("{:?}", self.conf.game_name);
    }
}
