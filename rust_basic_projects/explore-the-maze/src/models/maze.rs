use std::cmp;
use rand::Rng;
use tcod::colors::*;
use crate::models::entity::Entity;

// size of the maze
pub const MAZE_WIDTH: i32 = 80;
pub const MAZE_HEIGHT: i32 = 45;

// colors of the tiles - wall and ground
pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };

pub const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

// max num of room + max/min size of rooms
const MAX_ROOMS: i32 = 30;
pub const ROOM_MIN_SIZE: i32 = 5;
pub const ROOM_MAX_SIZE: i32 = 10;

// max num of monsters in each room
const MAX_MONSTERS_IN_ROOM: i32 = 3;

// player index in entities vector
// player will always be the first Entity
pub const PLAYER: usize = 0;

// custom type Maze - two dimentional array / jagged array
pub type Maze = Vec<Vec<Tile>>;

// the main game Entity
// maze is the map to be explored - a jagged array
pub struct Game {
    pub maze: Maze
}

// A tile of the maze and its properties
// clone & Copy - copy values as arguments instead of borrow
// Debug - print tile content
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

// Room struct for a maze room
// Use Clone & Copy traits to not pass Room as a reference each time
#[derive(Clone, Copy, Debug)]
struct Room {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Room {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // find the xenter of each room 
    // e.g top left (10, 10), bottom right (20, 20) => center ((x1 + x2) / 2, (y1 + y2) / 2) = (15, 15)
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    // check if two rooms intersect
    // ensure no rooms go over each other
    pub fn intersects_with(&self, other: &Room) -> bool {
        // returns true if this intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

/// @title create_room
/// @author GeorgiKostadinovPro
/// @notice create a custom room in maze
/// @dev custom fn to create an empty custom room within maze ((x, y), (x + dx, y + dy))
fn create_room(maze: &mut Maze, room: Room) {
    // go through the tiles in the room and make them passable
    // from x + 1 and y1 + 1 so that only inside the room is empty, not the walls
    // A..B means A is inclused up to B (exclusive)
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            maze[x as usize][y as usize] = Tile::empty();
        }
    }
}

/// @title create_room
/// @author GeorgiKostadinovPro
/// @notice create a custom tunnel in maze
/// @dev custom fn to create an empty custom tunnel within maze
fn create_tunnel(maze: &mut Maze, x1: i32, x2: i32, y1: i32, y2: i32, is_horizontal: bool) {
    // the tunner can be horizontal or vertical - isHorizontal
    // isHorizontal - loop in maze only on rows, not cols
    // in rust jagged array x - cols, y - rows. In C# x - rows, y - cols
    // x1 and x2 are the start and end, y1 is the height, y2 = 0 not needed
    // !isHorizontal - tunnel is vertical loop only through cols, not rows
    // y1 and y2 are the start and end, x1 is the width, x2 = 0 not needed
    // min & max ensure that we always start with the smaller number (1, 5) is the same as (5, 1)
    // otherwise the for loop will not produce result
    if is_horizontal {
        for x in cmp::min(x1, x2)..cmp::max(x1, x2) {
            maze[x as usize][y1 as usize] = Tile::empty();
        }
    } else {
        for y in cmp::min(y1, y2)..cmp::max(y1, y2) {
            maze[x1 as usize][y as usize] = Tile::empty();
        }
    }
}

/// @title create_monsters
/// @author GeorgiKostadinovPro
/// @notice create monsters in maze on random
/// @dev custom fn to create monsters within maze on random
fn create_monsters(room: Room, entities: &mut Vec<Entity>) {
    // choose random number of monsters
    let monsters_count = rand::thread_rng().gen_range(0, MAX_MONSTERS_IN_ROOM + 1);

    for _ in 0..monsters_count {
        // choose random spot for curr monster
        // (x1 + 1, y1 + 1) => x2, y2 (exclusive)
        // monster is placed only within room
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // 80% chance of getting an orc
        // 20% - trolls
        let mut monster = if rand::random::<f32>() < 0.8 {  
            // create an orc
            Entity::new(x, y, 'o', DESATURATED_GREEN, "orc", true)
        } else {
            Entity::new(x, y, 'T', DARKER_GREEN, "troll", true)
        };

        monster.is_alive = true;
        entities.push(monster);
    }
}

/// @title create_maze
/// @author GeorgiKostadinovPro
/// @notice create a custom jagged maze
/// @dev custom fn to create a custom jagged maze (80 inner vectors with 45 Tiles each)
pub fn create_maze(entities: &mut Vec<Entity>) -> Maze {
    // fill map with "unblocked" tiles
    let mut maze = vec![vec![Tile::wall(); MAZE_HEIGHT as usize]; MAZE_WIDTH as usize];

    // after populating the vec => loop it and call create_room()
    let mut rooms = vec![];

    // generate rooms and tunnels on random
    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        // maze is 80x45 ensure room start (x1, y1) (x1 + w, y2 + h) <= borders
        let x = rand::thread_rng().gen_range(0, MAZE_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAZE_HEIGHT - h);

        // init a room
        let room = Room::new(x, y, w, h);

        // loop the other rooms and see if any intersect with this one
        // if ture any() will abort and return true
        let failed = rooms
            .iter()
            .any(|other_room| room.intersects_with(other_room));

        if failed {
            continue;
        }

        // insert the room in the maze with empty tiles
        create_room(&mut maze, room);

        // create monsters
        create_monsters(room, entities);

        // get the center of the room to place the player
        let (center_x, center_y) = room.center();

        // if this room is the first put the player inside
        // for every other room try to connect it via a tunnel to the previous one
        if rooms.is_empty() {
            // this is the first room, where the player starts at
            entities[PLAYER].set_pos(center_x, center_y);
        } else {
            // all rooms after the first:
            // connect it to the previous room with a tunnel

            // center coordinates of the previous room (the curr last room)
            let (prev_center_x, prev_center_y) = rooms[rooms.len() - 1].center();

            // toss a coin (random bool value -- either true or false)
            if rand::random() {
                // first move horizontally, then vertically
                create_tunnel(&mut maze, prev_center_x, center_x, prev_center_y, 0, true);
                create_tunnel(&mut maze, center_x, 0, prev_center_y, center_y, false);
            } else {
                // first move vertically, then horizontally
                create_tunnel(&mut maze, prev_center_x, 0, prev_center_y, center_y, false);
                create_tunnel(&mut maze, prev_center_x, center_x, center_y, 0, true);
            }
        }

        // add room and use it to check the next ones
        rooms.push(room);
    } 

    maze
}
