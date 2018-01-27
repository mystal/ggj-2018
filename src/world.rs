use std::iter::Enumerate;
use std::slice::Iter;

use cgmath::{self, Vector2, InnerSpace};
use midgar::{KeyCode, Midgar};
use std::fs::File;
use tiled::{self, PropertyValue};

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

pub struct Bone {
    pub pos: Vector2<u32>,
}

impl Bone {
    fn new(x: u32, y: u32) -> Self {
        Bone {
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
    pub map: tiled::Map,
    //pub tiles: Vec<Tile>,
    //pub width: u32,
    //pub height: u32,
}

impl Level {
    fn new(map: tiled::Map) -> Self {
        Level {
            map,
        }
    }

    fn width(&self) -> u32 {
        self.map.width
    }

    fn height(&self) -> u32 {
        self.map.height
    }

    fn is_valid(&self, x: u32, y: u32) -> bool {
        x < self.width() && y < self.height()
    }

    // Returns a tile ID
    fn get_tile(&self, x: u32, y: u32) -> u32 {
        if !self.is_valid(x, y) {
            return 0;
        }
        self.map.layers[0].tiles[y as usize][x as usize]
    }

    pub fn iter_tiles_diagonal(&self) -> DiagonalTileIterator {
        DiagonalTileIterator {
            level: &self,
            next_tile_pos: Vector2::new(0, 0),
            last_start_pos: Vector2::new(0, 0),
        }
    }

    //pub fn iter_tiles(&self) -> TileIterator {
    //    TileIterator {
    //        inner: self.tiles.iter().enumerate(),
    //        width: self.width as usize,
    //        height: self.height as usize,
    //    }
    //}
}

pub struct DiagonalTileIterator<'a> {
    level: &'a Level,
    next_tile_pos: Vector2<u32>,
    last_start_pos: Vector2<u32>,
}

impl<'a> Iterator for DiagonalTileIterator<'a> {
    // (tile_id, x, y)
    type Item = (u32, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the current x and y are valid, if not we are done.
        if !self.level.is_valid(self.next_tile_pos.x, self.next_tile_pos.y) {
            return None;
        }

        // Save them off to return the current tile.
        let (x, y) = (self.next_tile_pos.x, self.next_tile_pos.y);

        /* The general algorithm for walking diagonal strips:
         * Start at 0, 0
         * Outer loop:
         *   Move y down until can't anymore.
         *   Move x over until can't anymore.
         *   Done.
         * Inner loop:
         *   Move x, y up and to the right until can't anymore.
         **/

        if x < self.level.width() - 1 && y > 0 {
            // Move up and to the right.
            self.next_tile_pos = Vector2::new(x + 1, y - 1);
        } else if self.last_start_pos.y < self.level.height() - 1 {
            // Move to (0, last_start_pos.y + 1).
            self.last_start_pos = Vector2::new(0, self.last_start_pos.y + 1);
            self.next_tile_pos = self.last_start_pos;
        } else {
            // Move to (last_start_pos.x + 1, last_start_pos.y)
            self.last_start_pos = Vector2::new(self.last_start_pos.x + 1, self.last_start_pos.y);
            self.next_tile_pos = self.last_start_pos;
        }

        Some((self.level.get_tile(x, y), x, y))
    }
}

//pub struct TileIterator<'a> {
//    inner: Enumerate<Iter<'a, Tile>>,
//    width: usize,
//    height: usize,
//}
//
//impl<'a> Iterator for TileIterator<'a> {
//    type Item = (Tile, usize, usize);
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let tile = self.inner.next();
//        tile.map(|(i, tile)| {
//            let (x, y) = (i % self.width, i / self.width);
//            (*tile, x, y)
//        })
//    }
//}

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
    pub level: Level,
    pub mail: Mail,
}

impl GameWorld {
    pub fn new(map_name: &str, assets_path: &str) -> Self {
        let map = GameWorld::load_map(map_name, assets_path);
        let fox = GameWorld::load_fox(&map)
            .expect(&format!("Could not load \"sneky_fox\" from map {}", map_name));
        let mailbox = GameWorld::load_mailbox(&map)
            .expect(&format!("Could not load \"mailbox\" from map {}", map_name));
        let mail = GameWorld::load_mail(&map)
            .expect(&format!("Could not load \"mail\" from map {}", map_name));

        GameWorld {
            game_state: GameState::Running,
            fox,
            mailbox,
            mail,
            level: Level::new(map),
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
        let mut dx = 0;
        let mut dy = 0;
        match (midgar.input().was_key_pressed(KeyCode::Left),
            (midgar.input().was_key_pressed(KeyCode::Right)),
            (midgar.input().was_key_pressed(KeyCode::Up)),
            (midgar.input().was_key_pressed(KeyCode::Down))) {
            (true, false, false, false) => {
                dx = -1
            },
            (false, true, false, false) => {
                dx = 1
            },
            (false, false, true, false) => {
                dy = -1
            },
            (false, false, false, true) => {
                dy = 1
            },
            _ => {},
        };

        self.try_move_fox(dx, dy);

        // TODO: iterate through pugs and see if fox is in the square they are pointing to

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

        let tile_id = self.level.get_tile(new_pos.x, new_pos.y);
        if tile_id != 0 {
            self.fox.pos = new_pos;
            self.fox.dir = Vector2::new(dx, dy);
        }
    }
}
