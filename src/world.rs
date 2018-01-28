use std::fs::File;
use std::iter::Enumerate;
use std::path::Path;
use std::slice::Iter;

use cgmath::{self, Vector2, InnerSpace};
use midgar::{KeyCode, Midgar};
use tiled::{self, PropertyValue};

use config;
use sounds::{Sound, Sounds, AudioController};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_str(dir_str: &str) -> Option<Self> {
        match dir_str {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }

    fn from_vector2(dir_vec: Vector2<i32>) -> Option<Self> {
        match (dir_vec.x, dir_vec.y) {
            (0, -1) => Some(Direction::North),
            (0, 1) => Some(Direction::South),
            (1, 0) => Some(Direction::East),
            (-1, 0) => Some(Direction::West),
            _ => None
        }
    }

    fn to_vector2(&self) -> Vector2<i32> {
        match *self {
            Direction::North => Vector2::new(0, -1),
            Direction::South => Vector2::new(0, 1),
            Direction::East => Vector2::new(1, 0),
            Direction::West => Vector2::new(-1, 0),
        }
    }
}

pub struct Fox {
    pub pos: Vector2<u32>,
    pub dir: Direction,
    pub has_mail: bool,
    pub move_sound: Sound,
    pub bone: Option<Bone>
}

impl Fox {
    fn new(x: u32, y: u32, dir: Direction) -> Self {
        Fox {
            pos: Vector2::new(x, y),
            dir,
            has_mail: false,
            move_sound: Sounds::fox_move(),
            bone: None,
        }
    }
}

pub struct Pug {
    pub pos: Vector2<u32>,
    pub dir: Direction,
}

impl Pug {
    fn new(x: u32, y: u32, dir: Direction) -> Self {
        Pug {
            pos: Vector2::new(x, y),
            dir,
        }
    }

    fn attack(&mut self, fox_pos: Vector2<u32>) {
        self.pos = fox_pos;
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
    pub is_held: bool,
}

impl Bone {
    fn new(x: u32, y: u32) -> Self {
        Bone {
            pos: Vector2::new(x, y),
            is_held: false
        }
    }

    pub fn get_throwable_positions(&self, level: &Level) -> Vec<Vector2<u32>> {
        let mut vec = Vec::new();
        let rng_vec: Vec<isize> = vec!(-1, 0, 1);

        for i in rng_vec.clone() {
            for j in rng_vec.clone() {
                if i == 0 && j == 0 {
                    continue;
                }
                let (x, y) = (self.pos.x as isize + i, self.pos.y as isize + j);
                if level.has_tile(x as u32, y as u32) {
                    vec.push(Vector2::new(x as u32, y as u32));
                }
            }
        }

        vec
    }

    pub fn is_held(&self) -> bool {
        self.is_held
    }
}

pub struct Level {
    pub map: tiled::Map,
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

    fn has_tile(&self, x: u32, y: u32) -> bool {
        self.get_tile(x, y) != 0
    }

    pub fn iter_tiles_diagonal(&self) -> DiagonalTileIterator {
        DiagonalTileIterator {
            level: &self,
            next_tile_pos: Vector2::new(0, 0),
            last_start_pos: Vector2::new(0, 0),
        }
    }
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
    pub pugs: Vec<Pug>,
    pub bones: Vec<Bone>,
    sounds: Sounds,
    pub time: f32,

    assets_path: String,
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
        let pugs = GameWorld::load_pugs(&map);
        let bones = GameWorld::load_bones(&map);

        GameWorld {
            game_state: GameState::Running,
            fox,
            mailbox,
            mail,
            pugs,
            bones,
            level: Level::new(map),
            sounds: Sounds::new(),
            time: 0.0,

            assets_path: assets_path.into(),
        }
    }

    fn load_map(map_name: &str, assets_path: &str) -> tiled::Map {
        let map_str = format!("{}/tiled/maps/{}.tmx", assets_path, map_name);
        let map_path = Path::new(&map_str);
        let map = tiled::parse_file(map_path)
            .expect(&format!("Could nor parse map file: {}", map_str));
        map
    }

    fn load_fox(map: &tiled::Map) -> Option<Fox> {

        for object in &map.object_groups[0].objects {
            if object.obj_type == "sneky_fox" {
                let x = object.x as u32 / (map.tile_width / 2);
                let y = object.y as u32 / map.tile_height;
                let facing = match object.properties.get("facing") {
                    Some(&PropertyValue::StringValue(ref s)) => s,
                    _ => "north"
                };
                let dir = Direction::from_str(facing)
                    .unwrap_or(Direction::North);
                return Some(Fox::new(x, y, dir));
            }
        }
        None
    }

    fn load_mailbox(map: &tiled::Map) -> Option<Mailbox> {
        for object in &map.object_groups[0].objects {
            if object.obj_type == "mailbox" {
                let x = object.x as u32 / (map.tile_width / 2);
                let y = object.y as u32 / map.tile_height;
                return Some(Mailbox::new(x, y));
            }
        }
        None
    }

    fn load_mail(map: &tiled::Map) -> Option<Mail> {
        for object in &map.object_groups[0].objects {
            if object.obj_type == "mail" {
                let x = object.x as u32 / (map.tile_width / 2);
                let y = object.y as u32 / map.tile_height;
                return Some(Mail::new(x, y));
            }
        }
        None
    }

    fn load_pugs(map: &tiled::Map) -> Vec<Pug> {
        let mut v: Vec<Pug> = vec!();
        for object in &map.object_groups[0].objects {
            if object.obj_type == "pug" {
                let x = object.x as u32 / (map.tile_width / 2);
                let y = object.y as u32 / map.tile_height;
                let facing = match object.properties.get("facing") {
                    Some(&PropertyValue::StringValue(ref s)) => s,
                    _ => "south"
                };
                let dir = Direction::from_str(facing)
                    .unwrap_or(Direction::South);
                v.push(Pug::new(x, y, dir));
            }
        }
        v
    }

    fn load_bones(map: &tiled::Map) -> Vec<Bone> {
        let mut vec = Vec::new();
        for object in &map.object_groups[0].objects {
            if object.obj_type == "bone" {
                let x = object.x as u32 / (map.tile_width / 2);
                let y = object.y as u32 / map.tile_height;
                vec.push(Bone::new(x, y));
            }
        }

        vec
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        match self.game_state {
            GameState::Running => self.update_running(midgar, dt),
            GameState::GameOver => self.update_over(midgar, dt),
            GameState::Won => self.update_won(midgar, dt),
            _ => {}
        }
    }

    fn update_over(&mut self, _midgar: &Midgar, dt: f32) {
        self.time += dt;

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

        for pug in &mut self.pugs {
            // This is weird but probably ok, maybe clamp on negative numbers?
            if (pug.pos.cast::<i32>() + pug.dir.to_vector2()).cast::<u32>() == self.fox.pos {
                pug.attack(self.fox.pos);
                self.sounds.bark.play();
                self.game_state = GameState::GameOver;
            }
        }

        // Check if any bones are activated/held
        let bones = &mut self.bones;
        for ref mut bone in bones.iter_mut() {
            bone.is_held = self.fox.pos == bone.pos;
        }

        // Check if fox grabbed mail
        if !self.fox.has_mail && self.fox.pos == self.mail.pos {
            self.sounds.got_mail.play();
            self.fox.has_mail = true;
        }

        // Check for victory!
        if self.fox.pos == self.mailbox.pos && self.fox.has_mail {
            self.sounds.won_level.play();
            self.game_state = GameState::Won;
        }
    }

    fn update_won(&mut self, midgar: &Midgar, dt: f32) {
        // Move to the next level if Enter is pressed.
        if midgar.input().was_key_pressed(KeyCode::Return) {
            // Check if there's a level to load, otherwise reload the start stage.
            let (map, fox, mailbox, mail, pugs, bones) = {
                let next_level = match self.level.map.properties.get("next_level") {
                    Some(&PropertyValue::StringValue(ref next_level)) => next_level,
                    _ => config::START_LEVEL,
                };
                let map = GameWorld::load_map(next_level, &self.assets_path);
                let fox = GameWorld::load_fox(&map)
                    .expect(&format!("Could not load \"sneky_fox\" from map {}", next_level));
                let mailbox = GameWorld::load_mailbox(&map)
                    .expect(&format!("Could not load \"mailbox\" from map {}", next_level));
                let mail = GameWorld::load_mail(&map)
                    .expect(&format!("Could not load \"mail\" from map {}", next_level));
                let pugs = GameWorld::load_pugs(&map);
                let bones = GameWorld::load_bones(&map);
                (map, fox, mailbox, mail, pugs, bones)
            };

            // TODO: Look into incremental update of self?
            self.game_state = GameState::Running;
            self.fox = fox;
            self.mailbox = mailbox;
            self.level = Level::new(map);
            self.mail = mail;
            self.pugs = pugs;
            self.bones = bones;
        }
    }

    fn try_move_fox(&mut self, dx: i32, dy: i32) {
        // Don't try to move if we're not moving!
        if dx == 0 && dy == 0 {
            return;
        }

        // FIXME: Assuming walls will prevent going negative.
        let new_pos = Vector2::new((self.fox.pos.x as i32 + dx) as u32,
                                   (self.fox.pos.y as i32 + dy) as u32);

        // TODO: Consider allowing to change directions when trying to move into a wall.
        if self.level.has_tile(new_pos.x, new_pos.y) {
            self.fox.move_sound.play();
            let fox_delta = Vector2::new(dx, dy);
            self.fox.pos = new_pos;
            self.fox.dir = Direction::from_vector2(fox_delta)
                .expect(&format!("Unexpected fox delta {:?}", fox_delta));
        }
    }
}
