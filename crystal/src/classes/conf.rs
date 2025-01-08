use macroquad::prelude::*;

pub struct GameConfig {
    pub game_name: String,
    pub engine_version: [u16; 3],
    pub window_title: String,
    pub window_width: i32,
    pub window_height: i32,
    pub window_icon: Option<macroquad::miniquad::conf::Icon>,
}
