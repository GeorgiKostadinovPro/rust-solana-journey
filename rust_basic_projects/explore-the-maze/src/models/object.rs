use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};

use crate::models::maze::Maze;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    // constructor
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // move by the given amount
    // if wall return
    pub fn move_by(&mut self, maze: &Maze, dx: i32, dy: i32) {
        // add the new deltas to the current player x, y 
        // check that his next position is not a wall
        let isWall = maze[(self.x + dx) as usize][(self.y + dy) as usize].blocked;

        if isWall {
            return;
        }

        self.x += dx;
        self.y += dy;
    }

    // set the color and then draw the character that represents this object at its position
    // dyn highlights that Console is a trait and not a concrete type (such as a struct or enum)
    pub fn draw(&self, console: &mut dyn Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}
