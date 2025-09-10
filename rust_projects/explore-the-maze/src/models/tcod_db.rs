use tcod::console::{Root, Offscreen};
use tcod::map::{FovAlgorithm, Map};

// constants
pub const GAME_TITLE: &str = "Explore the Maze";
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 20;

// sizes and coordinates relevant for the GUI
pub const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

// Filed of View (default FOV algorithm)
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
// light walls or not
pub const FOV_LIGHT_WALLS: bool = true; 
pub const TORCH_RADIUS: i32 = 10;

// encapsulate libtcod related values
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen,
    pub gui_panel: Offscreen,
    pub fov: Map
}
