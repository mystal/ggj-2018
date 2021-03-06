use std::path::Path;

use cgmath::Vector2;
use midgar::{KeyCode, Midgar};
use tiled::{self, PropertyValue};

use config;
use sounds::{Sound, Sounds, AudioController};

const DISAPPEARING_TIME: f32 = 0.5;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiveState {
    Alive,
    Dead(f32), // How long we've been dead for.
}

pub struct Fox {
    pub state: LiveState,
    pub pos: Vector2<u32>,
    pub dir: Direction,
    pub has_mail: bool,
    pub move_sound: Sound,
    pub bone: Option<Bone>
}

impl Fox {
    fn new(x: u32, y: u32, dir: Direction) -> Self {
        Fox {
            state: LiveState::Alive,
            pos: Vector2::new(x, y),
            dir,
            has_mail: false,
            move_sound: Sounds::fox_move(),
            bone: None,
        }
    }
}

#[derive(PartialEq)]
pub enum PugState {
    Guarding,
    Suprised(Vector2<u32>),
    Alerted(Vector2<u32>, Vector2<u32>),
}

pub struct Pug {
    pub live_state: LiveState,
    pub pos: Vector2<u32>,
    pub dir: Direction,
    pub state: PugState,
}

impl Pug {
    fn new(x: u32, y: u32, dir: Direction) -> Self {
        Pug {
            live_state: LiveState::Alive,
            pos: Vector2::new(x, y),
            dir: dir,
            state: PugState::Guarding
        }
    }

    fn attack(&mut self, fox_pos: Vector2<u32>) {
        self.pos = fox_pos;
    }

    fn set_guarding(&mut self) {
        self.state = PugState::Guarding;
    }

    fn set_alerted(&mut self, next_pos: Vector2<u32>, bone_pos: Vector2<u32>) {
        self.state = PugState::Alerted(next_pos, bone_pos);
    }

    fn set_suprised(&mut self, bone_pos: Vector2<u32>) {
        self.state = PugState::Suprised(bone_pos);
    }

    fn get_watched_pos(&self) -> Option<Vector2<u32>> {
        let v = self.pos.cast::<i32>() + self.dir.to_vector2();

        if v.x < 0 || v.y < 0 {
            return None
        } 
        Some(v.cast::<u32>())
    }

    fn set_facing(&mut self, new_pos: Vector2<u32>) {
        let mut facing = new_pos.cast::<i32>() - self.pos.cast::<i32>();
        if facing.x < 0 {
            facing.x = -1
        } else if facing.x > 0 {
            facing.x = 1
        }

        if facing.y < 0 {
            facing.y = -1
        } else if facing.y > 0 {
            facing.y = 1
        }

        let dir = Direction::from_vector2(facing).expect("Set facing should be successful");
        self.dir = dir;
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
    pub is_selected: bool,
    pub is_used: bool,
    pub is_visible: bool,
    pub disappearing_time: f32,
}

impl Bone {
    fn new(x: u32, y: u32) -> Self {
        Bone {
            pos: Vector2::new(x, y),
            is_selected: false,
            is_used: false,
            is_visible: true,
            disappearing_time: 0.0,
        }
    }

    pub fn get_throwable_positions(&self, level: &Level) -> Vec<Vector2<u32>> {
        let mut vec = Vec::new();
        let rng_vec: Vec<(isize, isize)> = vec!((-1, 0), (1, 0), (0, -1), (0, 1));

        for i in rng_vec {
            let (x, y) = (self.pos.x as isize + i.0, self.pos.y as isize + i.1);
            if level.has_tile(x as u32, y as u32) {
                vec.push(Vector2::new(x as u32, y as u32));
            }
        }

        vec
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn is_used(&self) -> bool {
        self.is_used
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

pub struct Level {
    pub map: tiled::Map,
    pub level_name: String,
}

impl Level {
    fn new(map: tiled::Map, level_name: String) -> Self {
        Level {
            map,
            level_name,
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

        // returns the maybe 8 legal tiles surrounding a single tile
    fn get_adjacent_eight(&self, pos: Vector2<u32>) -> Vec<Vector2<u32>> {
        let mut vec = Vec::new();
        let rng_vec: Vec<(isize, isize)> = vec!((-1, 0), (1, 0),
                                                (0, -1), (0, 1),
                                                (-1, 1), (-1, -1),
                                                (1, 1), (1, -1));

        for i in rng_vec {
            let (x, y) = (pos.x as isize + i.0, pos.y as isize + i.1);
            if self.has_tile(x as u32, y as u32) {
                vec.push(Vector2::new(x as u32, y as u32));
            }
        }
        vec
    }

    // returns the maybe cardinal 4 legal tiles surrounding a single tile
    fn get_adjacent_four(&self, pos: Vector2<u32>) -> Vec<Vector2<u32>> {
        let mut vec = Vec::new();
        let rng_vec: Vec<(isize, isize)> = vec!((-1, 0), (1, 0),
                                                (0, -1), (0, 1));

        for i in rng_vec {
            let (x, y) = (pos.x as isize + i.0, pos.y as isize + i.1);
            if self.has_tile(x as u32, y as u32) {
                vec.push(Vector2::new(x as u32, y as u32));
            }
        }
        vec
    }

    fn apply_bone_to_pugs(&mut self, pugs: &mut Vec<Pug>, bone_pos: Vector2<u32>) {
        let tiles = self.get_adjacent_eight(bone_pos);

        for pug in pugs {
            for tile in &tiles {
                if pug.pos == *tile {
                    pug.set_suprised(bone_pos);
                }
            }
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
        let (level, fox, mailbox, mail, pugs, bones) = GameWorld::load_level(map_name, assets_path);

        GameWorld {
            game_state: GameState::StartMenu,
            fox,
            mailbox,
            mail,
            pugs,
            bones,
            level,
            sounds: Sounds::new(),
            time: 0.0,

            assets_path: assets_path.into(),
        }
    }

    fn load_level(level_name: &str, assets_path: &str) -> (Level, Fox, Mailbox, Mail, Vec<Pug>, Vec<Bone>) {
        let map = GameWorld::load_map(level_name, assets_path);
        let fox = GameWorld::load_fox(&map)
            .expect(&format!("Could not load \"sneky_fox\" from map {}", level_name));
        let mailbox = GameWorld::load_mailbox(&map)
            .expect(&format!("Could not load \"mailbox\" from map {}", level_name));
        let mail = GameWorld::load_mail(&map)
            .expect(&format!("Could not load \"mail\" from map {}", level_name));
        let pugs = GameWorld::load_pugs(&map);
        let bones = GameWorld::load_bones(&map);
        let level = Level::new(map, level_name.into());

        (level, fox, mailbox, mail, pugs, bones)
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
            GameState::StartMenu => self.update_start_menu(midgar, dt),
            GameState::Credits => self.update_credits(midgar, dt),
            GameState::Running => self.update_running(midgar, dt),
            GameState::GameOver => self.update_over(midgar, dt),
            GameState::Won => self.update_won(midgar, dt),
            _ => {}
        }
    }

    fn update_start_menu(&mut self, midgar: &Midgar, _dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::Return) {
            self.game_state = GameState::Running;
        }
    }

    fn update_credits(&mut self, midgar: &Midgar, _dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::Return) {
            self.game_state = GameState::StartMenu;
        }
    }

    fn update_over(&mut self, midgar: &Midgar, dt: f32) {
        // Restart the level if Enter is pressed.
        if midgar.input().was_key_pressed(KeyCode::Return) {
            let (level, fox, mailbox, mail, pugs, bones) =
                GameWorld::load_level(&self.level.level_name, &self.assets_path);

            // TODO: Look into incremental update of self?
            self.game_state = GameState::Running;
            self.fox = fox;
            self.mailbox = mailbox;
            self.level = level;
            self.mail = mail;
            self.pugs = pugs;
            self.bones = bones;
        } else if let LiveState::Dead(ref mut dead_time) = self.fox.state {
            *dead_time += dt;
        }
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        let mut dx = 0;
        let mut dy = 0;
        match (midgar.input().was_key_pressed(KeyCode::Left) || midgar.input().was_key_pressed(KeyCode::A),
            (midgar.input().was_key_pressed(KeyCode::Right) || midgar.input().was_key_pressed(KeyCode::D)),
            (midgar.input().was_key_pressed(KeyCode::Up) || midgar.input().was_key_pressed(KeyCode::W)),
            (midgar.input().was_key_pressed(KeyCode::Down) || midgar.input().was_key_pressed(KeyCode::S))) {
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

        let mut fox_has_bone = false;
        let movement_requested = dx != 0 || dy != 0;
        {
            // Check if any bones are activated/selected
            let bones = &mut self.bones;
            for ref mut bone in bones.iter_mut() {
                // Throw the bone
                if bone.is_selected  && movement_requested {
                    let new_pos = Vector2::new((bone.pos.x as isize + dx) as u32,
                                       (bone.pos.y as isize + dy) as u32);
                    if self.level.has_tile(new_pos.x, new_pos.y) {
                        bone.pos = new_pos;
                        bone.is_used = true;
                        bone.is_selected = false;
                        bone.disappearing_time = DISAPPEARING_TIME;
                        fox_has_bone = true;

                        self.level.apply_bone_to_pugs(&mut self.pugs, new_pos);
                    }
                }

                // Make the bone flicker if we just threw it
                if bone.is_used {
                    if bone.disappearing_time > 0.0 {
                        bone.disappearing_time -= dt;
                        bone.is_visible = (bone.disappearing_time * 100.0).round() % 10.0 == 0.0;
                    } else {
                        bone.is_visible = false;
                    }
                }

                // Make fox grab bone
                if self.fox.pos == bone.pos {
                    if !bone.is_used {
                        fox_has_bone = true;
                        bone.is_selected = true;
                    }
                }
            }
        }

        // this code is bad I am sorry - Justin
        let mut fox_has_moved = false;
        if !fox_has_bone {
            fox_has_moved = self.try_move_fox(dx as i32, dy as i32);
        }

        for pug in &mut self.pugs {
            match pug.live_state {
                LiveState::Alive => {
                    match pug.state {
                        // If the pug is guarding, only move when it sees a fox
                        PugState::Guarding => {
                            if fox_has_moved{
                                if let Some(watched_pos) = pug.get_watched_pos() {
                                    if watched_pos == self.fox.pos {
                                        pug.attack(self.fox.pos);
                                        self.sounds.bark.play();
                                        self.fox.state = LiveState::Dead(0.0);
                                        self.game_state = GameState::GameOver;
                                    }
                                }
                            }
                        },
                        PugState::Suprised(bone_pos) => {
                            let mut bone_four = self.level.get_adjacent_four(bone_pos);
                            bone_four.push(bone_pos);
                            if let Some(watched) = pug.get_watched_pos() {
                                if watched == bone_pos {
                                    pug.set_alerted(watched, bone_pos);
                                } else if bone_four.iter().find(|&&x| x == watched).is_some() {
                                    pug.set_alerted(watched, bone_pos);
                                } else {
                                    let mut has_alerted = false;
                                    let pug_four = self.level.get_adjacent_four(pug.pos);
                                    for pug_adj in pug_four {
                                        if !has_alerted {
                                            match bone_four.iter().find(|&&x| x == pug_adj) {
                                                Some(pos) => {
                                                    has_alerted = true;
                                                    pug.set_alerted(*pos, bone_pos);
                                                    pug.set_facing(*pos);
                                                },
                                                None => {},
                                            }
                                        }
                                    }
                                    if !has_alerted {                                    
                                        pug.set_guarding();
                                    }
                                }
                            }
                        },
                        PugState::Alerted(alert_pos, bone_pos) => {
                            if fox_has_moved {
                                pug.set_facing(alert_pos);
                                if fox_has_moved {
                                    pug.attack(alert_pos);

                                    if alert_pos == bone_pos {
                                        pug.set_guarding();
                                    } else {
                                        pug.set_alerted(bone_pos, bone_pos);
                                    }

                                    if alert_pos == self.fox.pos {
                                        self.sounds.bark.play();
                                        self.fox.state = LiveState::Dead(0.0);
                                        self.game_state = GameState::GameOver;
                                    }
                                }
                            }
                        },
                    }
                }
                LiveState::Dead(ref mut dead_time) => *dead_time += dt,
            }
        }

        // Remove pugs dead for more than a second.
        self.pugs.retain(|pug| match pug.live_state {
            LiveState::Dead(dead_time) if dead_time >= 1.0 => false,
            _ => true,
        });

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

    fn update_won(&mut self, midgar: &Midgar, _dt: f32) {
        // Move to the next level if Enter is pressed.
        if midgar.input().was_key_pressed(KeyCode::Return) {
            self.game_state = GameState::Running;

            // Check if there's a level to load, otherwise reload the start stage.
            let (level, fox, mailbox, mail, pugs, bones) = {
                let next_level = match self.level.map.properties.get("next_level") {
                    Some(&PropertyValue::StringValue(ref next_level)) => next_level,
                    _ => {
                        self.game_state = GameState::Credits;
                        config::START_LEVEL
                    }
                };
                GameWorld::load_level(next_level, &self.assets_path)
            };

            // TODO: Look into incremental update of self?
            self.fox = fox;
            self.mailbox = mailbox;
            self.level = level;
            self.mail = mail;
            self.pugs = pugs;
            self.bones = bones;
        }
    }

    fn try_move_fox(&mut self, dx: i32, dy: i32) -> bool{
        // Don't try to move if we're not moving!
        if dx == 0 && dy == 0 {
            return false;
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

            // Kill any pugs.
            for pug in &mut self.pugs {
                if new_pos == pug.pos && pug.live_state == LiveState::Alive {
                    pug.live_state = LiveState::Dead(0.0);
                    if !self.sounds.lost_level.is_playing() {
                        self.sounds.lost_level.play();
                    }
                }
            }
            return true;
        }
        return false;
    }
}
