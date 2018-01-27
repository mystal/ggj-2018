use std::iter::Enumerate;
use std::slice::Iter;

use cgmath::{self, Vector2, InnerSpace};
use midgar::{KeyCode, Midgar};

pub struct Fox {
    pub pos: Vector2<u32>,
}

impl Fox {
    fn new(x: u32, y: u32) -> Self {
        Fox {
            pos: Vector2::new(x, y),
        }
    }
}

pub struct Mailbox {
    pub pos: Vector2<u32>,
}

impl Mailbox {
    fn new(x: u32, y: u32) -> Self {
        Mailbox {
            pos: Vector2::new(x, y),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
    Floor,
    Empty,
}

pub struct Level {
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
}

impl Level {
    fn new() -> Self {
        use self::Tile::*;

        let tiles = vec![
            Floor, Empty, Empty,
            Floor, Floor, Floor,
            Floor, Empty, Empty,
            Floor, Empty, Empty,
        ];
        Level {
            tiles,
            width: 3,
            height: 4,
        }
    }

    fn get_tile(&self, x: u32, y: u32) -> Tile {
        if x >= self.width || y >= self.height {
            return Tile::Empty;
        }
        let index = (y * self.width) + x;
        // FIXME: Return an Option or Result.
        self.tiles[index as usize]
    }

    pub fn iter_tiles(&self) -> TileIterator {
        TileIterator {
            inner: self.tiles.iter().enumerate(),
            width: self.width as usize,
            height: self.height as usize,
        }
    }
}

pub struct TileIterator<'a> {
    inner: Enumerate<Iter<'a, Tile>>,
    width: usize,
    height: usize,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = (Tile, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.inner.next();
        tile.map(|(i, tile)| {
            let (x, y) = (i % self.width, i / self.width);
            (*tile, x, y)
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Running,
}

pub struct GameWorld {
    pub game_state: GameState,
    pub fox: Fox,
    pub mailbox: Mailbox,
    pub level: Level,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            game_state: GameState::Running,
            fox: Fox::new(0, 3),
            mailbox: Mailbox::new(0, 0),
            level: Level::new(),
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        match self.game_state {
            GameState::Running => self.update_running(midgar, dt),
        }
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        let dx = match (midgar.input().was_key_pressed(KeyCode::Left), (midgar.input().was_key_pressed(KeyCode::Right))) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let dy = match (midgar.input().was_key_pressed(KeyCode::Up), (midgar.input().was_key_pressed(KeyCode::Down))) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        self.try_move_fox(dx, dy);
    }

    fn try_move_fox(&mut self, dx: isize, dy: isize) {
        // FIXME: Assuming walls will prevent going negative.
        let new_pos = Vector2::new((self.fox.pos.x as isize + dx) as u32,
                                   (self.fox.pos.y as isize + dy) as u32);

        match self.level.get_tile(new_pos.x, new_pos.y) {
            Tile::Floor => {
                self.fox.pos = new_pos;
            }
            // New position is empty, don't do anything.
            Tile::Empty => {}
        }
    }
}
