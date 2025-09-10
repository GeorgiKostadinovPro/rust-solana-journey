use tcod::console::{Root, Offscreen};
use tcod::colors::{Color};
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

// message log bar
pub const MSG_X: i32 = BAR_WIDTH + 2;
pub const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
pub const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

// encapsulate libtcod related values
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen,
    pub gui_panel: Offscreen,
    pub fov: Map
}

// list of messages (name, color)
pub struct Messages {
    pub messages: Vec<(String, Color)>
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    // add the new message as a tuple, with the text and the color
    // We can pass both & and String as they both implement the Into trait for String
    // anything that can be converted to String can be passed as T e.g. &str
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }
}
