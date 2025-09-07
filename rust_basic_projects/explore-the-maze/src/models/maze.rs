use tcod::colors::Color;

// size of the maze
pub const MAZE_WIDTH: i32 = 80;
pub const MAZE_HEIGHT: i32 = 45;

// colors of the tiles wall and ground
pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150 };

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

    maze
}
