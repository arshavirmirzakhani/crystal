use super::conf::GameConfig;

trait Init {
    fn init(&self);
}

pub struct Game {
    pub conf: GameConfig,
}

impl Init for Game {
    fn init(&self) {
        println!("{:?}", self.conf.game_name);
    }
}
