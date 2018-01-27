use std::iter::Enumerate;
use std::slice::Iter;

use cgmath::{self, Vector2, InnerSpace};
use midgar::{KeyCode, Midgar};
use std::fs::File;
use tiled::{self, PropertyValue};

trait Animal {
    fn update_pos(&mut self, Vector2<u32>);
    fn update_dir(&mut self, isize, isize);
    // FIXME: Assuming walls will prevent going negative.
    fn try_move(&mut self, level: &Level, pos: &Vector2<u32>, dx: isize, dy: isize) {
        let new_pos = Vector2::new((pos.x as isize + dx) as u32,
                                   (pos.y as isize + dy) as u32);

        match level.get_tile(new_pos.x, new_pos.y) {
            Tile::Floor => {
                self.update_pos(new_pos);
                self.update_dir(dx, dy);
            }
            // New position is empty, don't do anything.
            Tile::Empty => {
            }
        };
    }
}

pub struct Fox {
    pub pos: Vector2<u32>,
    pub dir: Vector2<isize>,
    pub has_mail: bool,
}

impl Fox {
    fn new(x: u32, y: u32) -> Self {
        Fox {
            pos: Vector2::new(x, y),
            dir: Vector2::new(1, 1),
            has_mail: false,
        }
    }
}

impl Animal for Fox {
    fn update_pos(&mut self, new_pos: Vector2<u32>) {
        self.pos = new_pos;
    }
    fn update_dir(&mut self, x: isize, y: isize) {
        if x != 0 {
            self.dir.x = x;
        }
        if y != 0 {
            self.dir.y = y;
        }
    }
}

pub struct Pug {
    pub pos: Vector2<u32>,
    pub dir: Vector2<isize>,
}

impl Pug {
    fn new(x: u32, y: u32) -> Self {
        Pug {
            pos: Vector2::new(x, y),
            dir: Vector2::new(1, 1),
        }
    }
}

impl Animal for Pug {
    fn update_pos(&mut self, new_pos: Vector2<u32>) {
        self.pos = new_pos;
    }
    fn update_dir(&mut self, x: isize, y: isize) {
        if x != 0 {
            self.dir.x = x;
        }
        if y != 0 {
            self.dir.y = y;
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

pub struct Mail {
    pub pos: Vector2<u32>,
}

impl Mail {
    fn new(x: u32, y: u32) -> Self {
        Mail {
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
    StartMenu,
    Credits,
    HowToPlay,
    Running,
    Won,
    GameOver,
}

pub struct GameWorld {
    pub game_state: GameState,
    pub fox: Fox,
    pub mailbox: Mailbox,
    pub level: tiled::Map,
    pub mail: Mail,
}

impl GameWorld {
    pub fn new(map_name: &str, assets_path: &str) -> Self {
        let level = GameWorld::load_map(map_name, assets_path);
        let fox = GameWorld::load_fox(&level)
            .expect(&format!("Could not load \"sneky_fox\" from map {}", map_name));
        let mailbox = GameWorld::load_mailbox(&level)
            .expect(&format!("Could not load \"mailbox\" from map {}", map_name));
        let mail = GameWorld::load_mail(&level)
            .expect(&format!("Could not load \"mail\" from map {}", map_name));

        GameWorld {
            game_state: GameState::Running,
            fox: fox,
            mailbox: mailbox,
            mail: mail,
            level: level,
        }
    }

    fn load_map(map_name: &str, assets_path: &str) -> tiled::Map {
        let map_path = format!("{}/tiled/maps/{}.tmx", assets_path, map_name);
        let map_file = File::open(&map_path)
            .expect(&format!("Could not open map path: {}", map_path));
        let map = tiled::parse(map_file)
            .expect(&format!("Could nor parse map file: {}", map_path));
        map
    }

    fn load_fox(map: &tiled::Map) -> Option<Fox> {
        for object in &map.object_groups[0].objects {
            if object.obj_type == "sneky_fox" {
                let x = object.x as u32 / map.tile_width;
                let y = object.y as u32 / map.tile_height;
                return Some(Fox::new(x, y));
            }
        }
        None
    }

    fn load_mailbox(map: &tiled::Map) -> Option<Mailbox> {
        for object in &map.object_groups[0].objects {
            if object.obj_type == "mailbox" {
                let x = object.x as u32 / map.tile_width;
                let y = object.y as u32 / map.tile_height;
                return Some(Mailbox::new(x, y));
            }            
        }
        None
    }

    fn load_mail(map: &tiled::Map) -> Option<Mail> {
        for object in &map.object_groups[0].objects {
            if object.obj_type == "mail" {
                let x = object.x as u32 / map.tile_width;
                let y = object.y as u32 / map.tile_height;
                return Some(Mail::new(x, y));
            }            
        }
        None
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        match self.game_state {
            GameState::Running => self.update_running(midgar, dt),
            _ => {}
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

        // Check for victory!
        if self.fox.pos == self.mailbox.pos && self.fox.has_mail {
            self.game_state = GameState::Won;
        }

        if self.fox.pos == self.mail.pos {
            self.fox.has_mail = true;
        }
    }

    fn try_move_fox(&mut self, dx: isize, dy: isize) {
        // FIXME: Assuming walls will prevent going negative.
        let new_pos = Vector2::new((self.fox.pos.x as isize + dx) as u32,
                                   (self.fox.pos.y as isize + dy) as u32);

        let tile =  self.level.layers[0].tiles[new_pos.y as usize][new_pos.x as usize];
        if tile != 0 {
            self.fox.pos = new_pos;
        }
    }
}
