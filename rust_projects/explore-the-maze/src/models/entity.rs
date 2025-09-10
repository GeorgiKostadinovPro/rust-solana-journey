use tcod::colors::*;
use tcod::console::{Console, BackgroundFlag};

use crate::models::maze::{Game};
use crate::models::util::*;

// deriving PartialEq lets us use == and != to compare the enums together
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit
}

// a callback to call into when entity (fighter) dies
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster
}

// fn to invoke the on death callback depending on the type of entity which died
// if player => player_death, monster => monster_death
impl DeathCallback {
    fn player_death(player: &mut Entity, game: &mut Game) {
        // the game ended
        game.messages.add("You died!", RED);

        // for added effect, transform the player into a corpse!
        player.char = '%';
        player.color = DARK_RED;
        player.is_alive = false;
    }

    fn monster_death(monster: &mut Entity, game: &mut Game) {
        // transform it into a corpse
        // it doesn't block, can't be attacked and doesn't move
        game.messages.add(format!("{} is dead!", monster.name), ORANGE);
        monster.char = '%';
        monster.color = DARK_RED;
        monster.is_blocking = false;
        monster.is_alive = false;
        monster.fighter = None;
        monster.name = format!("remains of {}", monster.name);
    }

    fn callback(self, entity: &mut Entity, game: &mut Game) {
        use DeathCallback::*;
        let callback: fn(&mut Entity, &mut Game) = match self {
            Player => DeathCallback::player_death,
            Monster => DeathCallback::monster_death
        };
        callback(entity, game);
    }
}

// combat-related properties and methods (monster, player, etc)
// on_death - if player died - end game, if monster - then add a corpse
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
// Entity may not be a fighter -> Option -> init passing None
#[derive(Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    pub name: String,  
    pub is_blocking: bool,  
    pub is_alive: bool,  
    pub fighter: Option<Fighter>
}

impl Entity {
    // constructor
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, is_blocking: bool) -> Self {
        Entity { 
            x, 
            y, 
            char, 
            color, 
            name: name.to_string(), 
            is_blocking, 
            is_alive: false, 
            fighter: None 
        }
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

    // player takes damage from monster
    // monster takes damane from player
    fn take_damage(&mut self, damage: i32, game: &mut Game) {
        // apply damage if possible
        // only if the entity is fighter, can it take damage
        // take as mut because we update the hp
        // cannot update both self and fighter in this if
        // because they are both "alive" and rust does not allow that
        // player.is_alive will be updated in the death callback
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }

            if fighter.hp <= 0 {
                fighter.on_death.callback(self, game);
            }
        }
    }

    // plater attacks monter
    // monster attacks player
    fn attack(&mut self, target: &mut Entity, game: &mut Game) {
        // a simple formula for attack damage
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            // make the target take some damage
            game.messages.add(
                format!(
                    "{} attacks {} for {} hit points.",
                    self.name, target.name, damage
                ),
                WHITE
            );

            target.take_damage(damage, game);
        } else {
            game.messages.add(
                format!(
                    "{} attacks {} but it has no effect!",
                    self.name, target.name
                ),
                WHITE
            );
        }
    }

    // move by the given amount
    // if wall return
    // self cannot be used because player is &mut, but entities is &, player is in the entitites
    // To guarantee memory safety and no data races, Rust’s references (& and &mut) have a few rules
    // One of them is that when you have a mutable borrow (player), you can’t have any other mutable or immutable borrows into the same data
    // solution: remove self and make entities &mut - read player from entities
    pub fn move_by(game: &mut Game, entities: &mut [Entity], idx: usize, dx: i32, dy: i32) {
        // add the new deltas to the current player x, y 
        // check that his next position is not a wall
        let (x, y) = entities[idx].get_pos();

        // check if tile is wall
        if game.maze[(x + dx) as usize][(y + dy) as usize].blocked {
            return;
        }

        // get the entity on (x + dx, y + dy) and if any fighter, attack it => take damage
        let maybe_target = entities
            .iter_mut()
            .enumerate()
            .find(|(i, e)| {
                *i != idx && e.get_pos() == (x + dx, y + dy) && e.fighter.is_some()
            });

        // Use target_idx not the target object because 
        // if you try to use target_entity and also access entities[idx] (the player) at the same time, 
        // Rust complains (two mutable borrows of the same slice)
        // Fix: split_at_mut works by splitting one slice into two non-overlapping slices, 
        // tricking Rust (safely) into treating them like different arrays
        if let Some((target_idx, _)) = maybe_target {
            // player attacks monster
            let (player, monster) = mut_two(entities, idx, target_idx);
            player.attack(monster, game);

            // monster retaliates
            if monster.is_alive {
                monster.attack(player, game);
            }

            return;
        }
        
        entities[idx].set_pos(x + dx, y + dy);
    }

    // set the color and then draw the character that represents this object at its position
    // dyn highlights that Console is a trait and not a concrete type (such as a struct or enum)
    pub fn draw(&self, console: &mut dyn Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}
