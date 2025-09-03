use tcod::colors::*;
use tcod::console::*;

// game title
const GAME_TITLE: &str = "Explore the Maze";

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// max 20 frames-per-second
const LIMIT_FPS: i32 = 20;

// encapsulate libtcod related values
struct Tcod {
    root: Root,
}

fn main() { 
    // limit the fps to 20
    tcod::system::set_fps(LIMIT_FPS);

    // create a new window
    // default values for not specified options
    let root: Root = Root::initializer()
    .font("arial10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title(GAME_TITLE)
    .init();

    // init the root options
    let mut tcod = Tcod { root };

    // start the game loop until the window is closed
    // the loop will be executed 20 times a second (limit fps = 20)
    while !tcod.root.window_closed() {
        // the color of all elements
        tcod.root.set_default_foreground(WHITE);
        // clear console of elements from previous frame
        tcod.root.clear();
        // draw player at coo (1, 1), ignore the default background color
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        // draw everything on the wondow at once
        tcod.root.flush();
        // necessary because libtcod handles the window manager’s events in the input processing code
        // without it the window_closed() will not work, crashing or hanging the game
        tcod.root.wait_for_keypress(true);
    }
}

