use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map};

// register modules in the crate
mod models;

// import modules from crate
use crate::models::maze::*;
use crate::models::entity::*;
use crate::models::tcod_db::*;

use crate::models::entity::PlayerAction::{TookTurn, DidntTakeTurn, Exit};

/// @title render_maze
/// @author GeorgiKostadinovPro
/// @notice render the whole maze with its elements and entities
/// @dev custom fn to render a custom jagged maze with its elements and entities
pub fn render_maze(tcod: &mut Tcod, game: &Game, entities: &[Entity]) {
    for entity in entities {
        entity.draw(&mut tcod.offscreen);
    }

    // go through all tiles, and set their background color
    // visit each inner vector
    for i in 0..MAZE_WIDTH {
        // visit each element in vector
        for j in 0..MAZE_HEIGHT {
            // if view is blocked then this is a wall
            let is_wall = game.maze[i as usize][j as usize].block_sight;

            // if wall drew it otherwise it is a ground tile
            if is_wall {
                tcod.offscreen.set_char_background(i, j, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.offscreen.set_char_background(i, j, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }   

    // blit the contents of "offscreen" to the root console and present it
    // blit(from, start coo, width and height of area to blit, to, start blit from coo, transparency)
    // From now on, the offscreen console Entity will represent only the map
    blit(&tcod.offscreen, (0, 0), (MAZE_WIDTH, MAZE_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);
}

/// @title handle_player_actions
/// @author GeorgiKostadinovPro
/// @notice keyboard handling fn
/// @dev custom fn to handle keyboard interaction
fn handle_player_actions(tcod: &mut Tcod, maze: &Maze, entities: &mut [Entity]) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key: Key = tcod.root.wait_for_keypress(true);

    // if player is dead do not allow to move
    let is_alive = entities[PLAYER].is_alive;
    
    // actions supported:
    // enter + alt - full screen
    // escape => close game
    // up, down, left, right => move player
    // toggle screen and exit - work whether player is alive/dead
    // for movement - is_alive must be true
    match (key, key.text(), is_alive) {
        // get only the action without any other fields (..)
        // without .. code will not compile because we have to specify each field
        (Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
            _
        ) => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            // this does not count as player action
            DidntTakeTurn
        }
        (
            Key { 
                code: Escape, .. 
            }, 
            _, 
            _
        ) => Exit,
        (Key 
            { 
                code: Up, .. 
            },
            _,
            true
        ) => {
            Entity::move_by(maze, entities, PLAYER, 0, -1); 
            TookTurn
        },
        (Key 
            { 
                code: Down, .. 
            },
            _,
            true
        ) => {
            Entity::move_by(maze, entities, PLAYER, 0, 1);
            TookTurn
        },
        (Key 
            { 
                code: Left, .. 
            },
            _,
            true
        ) => {
            Entity::move_by(maze, entities, PLAYER, -1, 0);
            TookTurn
        },
        (Key 
            { 
                code: Right, .. 
            },
            _,
            true
        ) => {
            Entity::move_by(maze, entities, PLAYER, 1, 0);
            TookTurn
        },
        _ => DidntTakeTurn
    }
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
    // maze is smaller than root console, the empty space will be used for healthy bar, messages, etc
    let offscreen = Offscreen::new(MAZE_WIDTH, MAZE_HEIGHT);

    // init a field of view map (tcod_db.rs for more docs)
    let fov = Map::new(MAZE_WIDTH, MAZE_HEIGHT);

    // init the root options
    let mut tcod = Tcod { root, offscreen, fov };    

    // init a player
    let mut player = Entity::new(0, 0, '@', WHITE, "go4ko", true);  
    player.is_alive = true;  
    
    // current entities
    let mut entities = vec![player];

    // init game and create a maze ref maze.rs for more docs
    // player will be placed in the center of the first generated room
    // monters will be placed within each generated room on random
    let game = Game { maze: create_maze(&mut entities) }; 

    // start the game loop until the window is closed
    // the loop will be executed 20 times a second (limit fps = 20)
    // golden rule for roguelikes turn-based:
    // 1. Render: clear screen => draw game on screen => flush to root
    // 2. Input: block until a key is pressed
    // 3. Update: match key and change player's coordinates
    // 4. Repeat
    while !tcod.root.window_closed() {
        // clear console of elements from previous frame
        tcod.offscreen.clear();

        render_maze(&mut tcod, &game, &entities);

        // flush to root so the window shows the frame
        tcod.root.flush();

        // handle actions and exit game if needed
        // entities are vec but fn accepts &mut [Entity] 
        // deref coercion - create a mutable slice - mutate elements inside, but resize vec
        let player_action = handle_player_actions(&mut tcod, &game.maze, &mut entities);
        if player_action == PlayerAction::Exit {
            break;
        }
    }
}
