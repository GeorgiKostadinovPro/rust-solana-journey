use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};

use crate::models::maze::Maze;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    pub name: String,  
    pub is_blocking: bool,  
    pub alive: bool,  
}

impl Entity {
    // constructor
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, is_blocking: bool) -> Self {
        Entity { x, y, char, color, name: name.to_string(), is_blocking, alive: false }
    }

    // getter
    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    // setter
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // move by the given amount
    // if wall return
    pub fn move_by(&mut self, maze: &Maze, dx: i32, dy: i32) {
        // add the new deltas to the current player x, y 
        // check that his next position is not a wall
        let is_wall = maze[(self.x + dx) as usize][(self.y + dy) as usize].blocked;

        if is_wall {
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
