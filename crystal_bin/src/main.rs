use classes::{conf::GameConfig, game::Game};
use crystal::*;
use macroquad::prelude::*;

#[macroquad::main("crystal ball test")]
async fn main() {
    let game_conf: GameConfig = GameConfig {
        game_name: "game".to_owned(),
        engine_version: [0, 1, 0],
        window_title: "test game".to_owned(),
        window_width: 800,
        window_height: 600,
        window_icon: None,
    };

    let game: Game = Game { conf: game_conf };

    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
