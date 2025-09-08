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
pub fn render_maze(tcod: &mut Tcod, game: &Game, entities: &[Entity], fov_recompute: bool) {
    if fov_recompute {
        // recompute FOV if needed (the player has moved)
        // move fov with the player
        let player = &entities[0];

        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    // go through all tiles, and set their background color
    // visit each inner vector
    for x in 0..MAZE_WIDTH {
        // visit each element in vector
        for y in 0..MAZE_HEIGHT {
            // check if location is visible
            let is_visible = tcod.fov.is_in_fov(x, y);

            // if view is blocked then this is a wall
            let is_wall = game.maze[x as usize][y as usize].block_sight;

            // if wall or ground is visible then lighten them
            // otherwise is not visible set dark colors
            let color = match (is_visible, is_wall) {
                // outside of field of view:
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside fov:
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND
            };

            tcod.offscreen
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }   

    // draw all entities in the list
    // if entity is in FOV then draw it
    for entity in entities {
        if tcod.fov.is_in_fov(entity.x, entity.y) {
            entity.draw(&mut tcod.offscreen);
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

    // populate the FOV map, according to the generated maze
    // the libtcod FOV module needs to know which tiles block sight
    // ToDo: extract in fn()
    for x in 0..MAZE_WIDTH {
        for y in 0..MAZE_HEIGHT {
            tcod.fov.set(
                x,
                y,
                !game.maze[x as usize][y as usize].block_sight,
                !game.maze[x as usize][y as usize].blocked,
            );
        }
    }

    // FOV needs to be recomputed — but only if the player moves or a tile changes
    // force FOV "recompute" first time through the game loop
    // using (-1, -1) to make sure FOV gets computed on the first time through the loop
    let mut player_previous_position = (-1, -1);

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

        // recompute the fov as player moves around
        // (-1, -1) != (0, 0) => recompute the fov based on the player location
        // (0, 0) != (x, y) => player has moved => move the fov with him
        let fov_recompute = player_previous_position != (entities[0].x, entities[0].y);

        render_maze(&mut tcod, &game, &entities, fov_recompute);

        // flush to root so the window shows the frame
        tcod.root.flush();

        // (0, 0) on the first run then player (x, y) will change from keyboard action
        // (x, y) on the second run then player (x, y) will change again
        player_previous_position = (entities[0].x, entities[0].y);

        // handle actions and exit game if needed
        // entities are vec but fn accepts &mut [Entity] 
        // deref coercion - create a mutable slice - mutate elements inside, but resize vec
        let player_action = handle_player_actions(&mut tcod, &game.maze, &mut entities);
        if player_action == PlayerAction::Exit {
            break;
        }
    }
}
