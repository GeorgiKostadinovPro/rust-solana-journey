use std::cmp;
use rand::Rng;
use tcod::colors::Color;

// size of the maze
pub const MAZE_WIDTH: i32 = 80;
pub const MAZE_HEIGHT: i32 = 45;

// colors of the tiles wall and ground
pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150 };

// max num of room + max/min size of rooms
const MAX_ROOMS: i32 = 30;
pub const ROOM_MIN_SIZE: i32 = 5;
pub const ROOM_MAX_SIZE: i32 = 10;

// custom type Maze - two dimentional array / jagged array
pub type Maze = Vec<Vec<Tile>>;

// the main game object
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
fn create_room(room: Room, maze: &mut Maze) {
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
fn create_tunnel(maze: &mut Maze, x1: i32, x2: i32, y1: i32, y2: i32, isHorizontal: bool) {
    // the tunner can be horizontal or vertical - isHorizontal
    // isHorizontal - loop in maze only on rows, not cols
    // in rust jagged array x - cols, y - rows. In C# x - rows, y - cols
    // x1 and x2 are the start and end, y1 is the height, y2 = 0 not needed
    // !isHorizontal - tunner is vertical loop only through cols, not rows
    // y1 and y2 are the start and end, x1 is the width, x2 = 0 not needed
    // min & max ensure that we always start with the smaller number (1, 5) is the same as (5, 1)
    // otherwise the for loop will not produce result
    if isHorizontal {
        for x in cmp::min(x1, x2)..cmp::max(x1, x2) {
            maze[x as usize][y1 as usize] = Tile::empty();
        }
    } else {
        for y in cmp::min(y1, y2)..cmp::max(y1, y2) {
            maze[x1 as usize][y as usize] = Tile::empty();
        }
    }
}

/// @title create_maze
/// @author GeorgiKostadinovPro
/// @notice create a custom jagged maze
/// @dev custom fn to create a custom jagged maze (80 inner vectors with 45 Tiles each)
pub fn create_maze() -> Maze {
    // fill map with "unblocked" tiles
    let mut maze = vec![vec![Tile::wall(); MAZE_HEIGHT as usize]; MAZE_WIDTH as usize];

    // place some rooms - will randomize later
    let room1 = Room::new(20, 15, 10, 15);
    let room2 = Room::new(50, 15, 10, 15);

    create_room(room1, &mut maze);
    create_room(room2, &mut maze);

    create_tunnel(&mut maze, 25, 55, 23, 0, true);

    maze
}
