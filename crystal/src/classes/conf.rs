// this class is holding the required information for the game classe

pub struct GameConfig {
    pub game_name: String,
    pub engine_version: [u16; 3], // a SemVer array for the engine versioning [MAJOR, MINOR, PATCH]
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}
