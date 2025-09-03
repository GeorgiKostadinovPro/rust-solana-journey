use tcod::colors::*;
use tcod::console::*;

// register modules in the crate
mod models;

// import modules from crate
use crate::models::tcod_db::*;
use crate::models::object::Object;

/// @title handle_player_actions
/// @author GeorgiKostadinovPro
/// @notice keyboard handling fn
/// @dev custom fn to handle keyboard interaction
fn handle_player_actions(tcod: &mut Tcod, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key: Key = tcod.root.wait_for_keypress(true);
    
    // actions supported:
    // enter + alt - full screen
    // escape => close game
    // up, down, left, rright => move player
    match key {
        // get only the action without any other fields (..)
        // without .. code will not compile because we have to specify each field
        Key {
                code: Enter,
                alt: true,
                ..
            } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        _ => {}
    }

    false
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

    // use offscreen console for transparency effects and rendring part of the main root window
    let offscreen = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // init the root options
    let mut tcod = Tcod { root, offscreen };

    // init a player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    // init an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    // current entities
    let mut entities = [player, npc];

    // start the game loop until the window is closed
    // the loop will be executed 20 times a second (limit fps = 20)
    while !tcod.root.window_closed() {
        // clear console of elements from previous frame
        tcod.offscreen.clear();

        // draw entities to offscreen console
        for entity in &entities {
            entity.draw(&mut tcod.offscreen);
        }

        // blit the contents of "offscreen" to the root console and present it
        // blit(from, start coo, width and height of area to blit, to, start blit from coo, transparency)
        blit(&tcod.offscreen, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);

        // draw everything on the wondow at once
        tcod.root.flush();
        // necessary because libtcod handles the window manager’s events in the input processing code
        // without it the window_closed() will not work, crashing or hanging the game
        tcod.root.wait_for_keypress(true);

        // handle actions and exit game if needed
        let exit = handle_player_actions(&mut tcod, &mut entities[0]);
        if exit {
            break;
        }
    }
}
