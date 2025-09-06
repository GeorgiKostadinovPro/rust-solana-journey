use tcod::colors::Color;

// size of the maze
const MAZE_WIDTH: i32 = 80;
const MAZE_HEIGHT: i32 = 45;

// colors of the tiles wall and ground
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150 };

// custom type Maze - two dimentional array / jagged array
type Maze = Vec<Vec<Tile>>;

struct Game {
    map: Maze
}

// A tile of the maze and its properties
// clone & Copy - copy values as arguments instead of borrow
// Debug - print tile content
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
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

/// @title create_maze
/// @author GeorgiKostadinovPro
/// @notice create a custom jagged maze
/// @dev custom fn to create a custom jagged maze (80 inner vectors with 45 Tiles each)
pub fn create_maze() -> Maze {
    // fill map with "unblocked" tiles
    let mut maze = vec![vec![Tile::empty(); MAZE_HEIGHT as usize]; MAZE_WIDTH as usize];

    // place some walls - will randomize later
    maze[30][22] = Tile::wall();
    maze[50][22] = Tile::wall();

    maze
}
