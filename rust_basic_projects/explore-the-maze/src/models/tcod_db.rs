use tcod::console::{Root, Offscreen};

// constants
pub const GAME_TITLE: &str = "Explore the Maze";
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 20;

// encapsulate libtcod related values
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen
}
