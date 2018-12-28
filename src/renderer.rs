use std::rc::Rc;

use cgmath::{self, Matrix4, Vector2};
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, Sprite, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::{TextureRegion, TextureRegionHolder};
use tiled::Tileset;

use config;
use world::*;

pub struct GameRenderer<'a> {
    projection: Matrix4<f32>,
    ui_projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    text: TextRenderer,

    tiles: Vec<TextureRegion>,
    background: TextureRegion,
    instructions: TextureRegion,
    title: TextureRegion,
    win_screen: TextureRegion,
    sneky_fox: Sprite<'a>,
    sneky_fox_back: Sprite<'a>,
    sneky_fox_with_mail: Sprite<'a>,
    sneky_fox_with_mail_back: Sprite<'a>,
    mailbox: Sprite<'a>,
    letter_1: Sprite<'a>,
    bone: Sprite<'a>,
    pug_idle_anim: Animation,
    pug_back_idle_anim: Animation,
    shadow: TextureRegion,

    font: Font<'a>,

    game_time: f32,
}

impl<'a> GameRenderer<'a> {
    pub fn new(midgar: &Midgar, tilesets: &[Tileset]) -> Self {
        // Load textures.
        let tiles = load_tiles(tilesets, midgar);
        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/background.png", false));
            TextureRegion::new(texture)
        };
        let instructions = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/instructions_background.png", false));
            TextureRegion::new(texture)
        };
        let title = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/title.png", false));
            TextureRegion::new(texture)
        };
        let win_screen = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/you_are_winner.png", false));
            TextureRegion::new(texture)
        };
        let sneky_fox = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_front.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.8, 0.8));
            let size = sprite.size();
            sprite.set_origin(Vector2::new(92.0 / size.x as f32, 80.0 / size.y as f32));
            sprite
        };
        let sneky_fox_back = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_back.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.8, 0.8));
            let size = sprite.size();
            sprite.set_origin(Vector2::new(92.0 / size.x as f32, 80.0 / size.y as f32));
            sprite
        };
        let sneky_fox_with_mail = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_front_with_mail.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.8, 0.8));
            let size = sprite.size();
            sprite.set_origin(Vector2::new(92.0 / size.x as f32, 80.0 / size.y as f32));
            sprite
        };
        let sneky_fox_with_mail_back = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_back_with_mail.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.8, 0.8));
            let size = sprite.size();
            sprite.set_origin(Vector2::new(92.0 / size.x as f32, 80.0 / size.y as f32));
            sprite
        };
        let mailbox = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/mailbox.png", false));
            let mut sprite = Sprite::new(texture);
            let size = sprite.size();
            sprite.set_origin(Vector2::new(44.0 / size.x as f32, 79.0 / size.y as f32));
            sprite
        };
        let letter_1 = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/letter_1.png", false));
            let mut sprite = Sprite::new(texture);
            let size = sprite.size();
            sprite.set_origin(Vector2::new(32.0 / size.x as f32, 47.0 / size.y as f32));
            sprite
        };
        let bone = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/bone.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.6, 0.6));
            sprite
        };
        let pug_idle_anim = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/pug_front_idle.png", false));
            let mut anim = Animation::new(0.1, &TextureRegion::split(texture, (125, 151)))
                .unwrap();
            anim.play_mode = PlayMode::Loop;
            anim
        };
        let pug_back_idle_anim = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/pug_back_idle.png", false));
            let mut anim = Animation::new(0.1, &TextureRegion::split(texture, (125, 151)))
                .unwrap();
            anim.play_mode = PlayMode::Loop;
            anim
        };
        let shadow = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/shadow.png", false));
            TextureRegion::new(texture)
        };

        let projection = cgmath::ortho(-(config::GAME_SIZE.x as f32 / 2.0), config::GAME_SIZE.x as f32 / 2.0,
                                       config::GAME_SIZE.y as f32 / 2.0, -(config::GAME_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);
        let ui_projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                       config::SCREEN_SIZE.y as f32, 0.0,
                                       -1.0, 1.0);

        GameRenderer {
            projection,
            ui_projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
            text: TextRenderer::new(midgar.graphics().display()),

            tiles,
            background,
            instructions,
            title,
            win_screen,
            sneky_fox,
            sneky_fox_back,
            sneky_fox_with_mail,
            sneky_fox_with_mail_back,
            mailbox,
            letter_1,
            bone,
            pug_idle_anim,
            pug_back_idle_anim,
            shadow,

            font: text::load_font_from_path("assets/fonts/Indie_Flower/IndieFlower.ttf"),

            game_time: 0.0,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        self.game_time += dt;

        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Linear)
            .alpha(true);

        match world.game_state {
            GameState::StartMenu => {
                self.draw_title(&mut target, draw_params);
                self.draw_ui(world, &mut target, draw_params);
            }
            GameState::Running => {
                self.draw_world(world, &mut target, draw_params);
                self.draw_ui(world, &mut target, draw_params);
            }
            GameState::Credits => {
                self.draw_credits(&mut target, draw_params);
                self.draw_ui(world, &mut target, draw_params);
            }
            _ => {
                self.draw_world(world, &mut target, draw_params);
                self.draw_ui(world, &mut target, draw_params);
            }
        }

        target.finish().unwrap();
    }

    fn draw_title<S: Surface>(&mut self, target: &mut S, draw_params: SpriteDrawParams) {
        // Draw background and title image.
        self.sprite.set_projection_matrix(self.ui_projection);
        self.sprite.draw(&self.background.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);
        self.sprite.draw(&self.title.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);
    }

    fn draw_credits<S: Surface>(&mut self, target: &mut S, draw_params: SpriteDrawParams) {
        // Draw background and title image.
        self.sprite.set_projection_matrix(self.ui_projection);
        self.sprite.draw(&self.background.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);
        self.sprite.draw(&self.win_screen.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);
    }

    fn draw_world<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        // Draw background.
        self.sprite.set_projection_matrix(self.ui_projection);

        // FIXME: Don't hard code this, but whatever.
        if world.level.level_name == "mockup" {
            // Draw the instructions background for the mockup level.
            self.sprite.draw(&self.instructions.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                             draw_params, target);
        } else {
            // Draw the default background.
            self.sprite.draw(&self.background.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                             draw_params, target);
        }

        self.sprite.set_projection_matrix(self.projection);
        self.shape.set_projection_matrix(self.projection);

        // Draw tiles.
        self.draw_tiles(world, target, draw_params);

        // TODO: Collect game objects in draw-order.

        // TODO: Draw object shadows top-down, left-right in the iso view.

        // TODO: Draw game objects top-down, left-right in the iso view.
        self.draw_pugs(world, target, draw_params);
        self.draw_mailbox(world, target, draw_params);
        self.draw_bones(world, target, draw_params);
        self.draw_mail(world, target, draw_params);
        self.draw_fox(world, target, draw_params);
    }

    fn draw_tiles<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let tile_width = world.level.map.tile_width as f32;
        let tile_height = world.level.map.tile_height as f32;

        for (tile, x, y) in world.level.iter_tiles_diagonal() {
            // Don't draw empty tiles.
            if tile == 0 {
                continue;
            }

            // Draw tile texture.
            let texture = &self.tiles[(tile - 1) as usize];
            let (draw_x, draw_y) = grid_to_isometric(x, y, tile_width, tile_height);
            self.sprite.draw(&texture.draw(draw_x, draw_y),
                             draw_params, target);
        }
    }

    fn draw_fox<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let tile_width = world.level.map.tile_width as f32;
        let tile_height = world.level.map.tile_height as f32;

        let sprite = match (world.fox.has_mail, world.fox.dir) {
            (false, Direction::South) | (false, Direction::East) => &mut self.sneky_fox,
            (false, Direction::North) | (false, Direction::West) => &mut self.sneky_fox_back,
            (true, Direction::South) | (true, Direction::East) => &mut self.sneky_fox_with_mail,
            (true, Direction::North) | (true, Direction::West) => &mut self.sneky_fox_with_mail_back,
        };
        let flip_x = world.fox.dir == Direction::East || world.fox.dir == Direction::North;
        let (flip_y, dead_offset) = match world.fox.state {
            LiveState::Dead(dead_time) => (true, dead_time * config::FALL_SPEED),
            _ => (false, 0.0),
        };
        let pos = world.fox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        // NOTE: Subtract 8 pixels to align to the center of the squares.
        sprite.set_flip_x(flip_x);
        sprite.set_flip_y(flip_y);

        // Draw shadow if alive.
        if world.fox.state == LiveState::Alive {
            self.sprite.draw(&self.shadow.draw(draw_x, draw_y - 8.0 + dead_offset), draw_params, target);
        }
        sprite.set_position(Vector2::new(draw_x, draw_y - 8.0 + dead_offset));
        self.sprite.draw(sprite, draw_params, target);
    }

    fn draw_mail<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        if !world.fox.has_mail {
            let tile_width = world.level.map.tile_width as f32;
            let tile_height = world.level.map.tile_height as f32;

            let pos = world.mail.pos;
            let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);

            self.sprite.draw(&self.shadow.draw(draw_x, draw_y - 8.0), draw_params, target);
            // NOTE: Subtract 8 pixels to align to the center of the squares.
            self.letter_1.set_position(Vector2::new(draw_x, draw_y - 8.0));
            self.sprite.draw(&self.letter_1, draw_params, target);
        }
    }

    fn draw_bones<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let tile_width = world.level.map.tile_width as f32;
        let tile_height = world.level.map.tile_height as f32;

        for bone in &world.bones {
            if bone.is_visible() {
                if bone.is_selected() {
                    // Draw locations where fox can throw bone
                    let v = bone.get_throwable_positions(&world.level);
                    for pos in &v {
                        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
                        self.bone.set_position(Vector2::new(draw_x, draw_y - 8.0));
                        self.bone.set_color(cgmath::vec4(1.0, 1.0, 1.0, 0.2));
                        self.sprite.draw(&self.bone, draw_params, target);
                    }
                } else {
                    // Draw bone
                    let pos = bone.pos;
                    let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);

                    self.sprite.draw(&self.shadow.draw(draw_x, draw_y - 8.0), draw_params, target);
                    // NOTE: Subtract 8 pixels to align to the center of the squares.
                    self.bone.set_position(Vector2::new(draw_x, draw_y - 8.0));
                    self.bone.set_color(cgmath::vec4(1.0, 1.0, 1.0, 1.0));
                    self.sprite.draw(&self.bone, draw_params, target);
                }
            }
        }
    }

    fn draw_mailbox<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let tile_width = world.level.map.tile_width as f32;
        let tile_height = world.level.map.tile_height as f32;

        let pos = world.mailbox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);

        self.sprite.draw(&self.shadow.draw(draw_x, draw_y - 8.0), draw_params, target);
        // NOTE: Subtract 8 pixels to align to the center of the squares.
        self.mailbox.set_position(Vector2::new(draw_x, draw_y - 8.0));
        self.sprite.draw(&self.mailbox, draw_params, target);
    }

    fn draw_pugs<S: Surface>(&mut self, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let tile_width = world.level.map.tile_width as f32;
        let tile_height = world.level.map.tile_height as f32;

        for pug in &world.pugs {
            let anim = match pug.dir {
                Direction::South | Direction::East => &mut self.pug_idle_anim,
                Direction::North | Direction::West => &mut self.pug_back_idle_anim,
            };
            let flip_x = pug.dir == Direction::East || pug.dir == Direction::North;
            let pos = pug.pos;
            let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
            let (flip_y, dead_offset) = match pug.live_state {
                LiveState::Dead(dead_time) => (true, dead_time * config::FALL_SPEED),
                _ => (false, 0.0),
            };

            // Draw shadow if alive.
            if pug.live_state == LiveState::Alive {
                self.sprite.draw(&self.shadow.draw(draw_x, draw_y - 8.0 + dead_offset), draw_params, target);
            }

            // NOTE: Subtract 8 pixels to align to the center of the squares.
            let mut sprite = anim.current_key_frame(self.game_time)
                .draw(draw_x, draw_y - 8.0 + dead_offset);
            sprite.set_flip_x(flip_x);
            sprite.set_flip_y(flip_y);
            let size = sprite.size();
            sprite.set_origin(Vector2::new(62.0 / size.x as f32, 145.0 / size.y as f32));
            self.sprite.draw(&sprite, draw_params, target);
        }
    }

    fn draw_ui<S: Surface>(&mut self, world: &GameWorld, target: &mut S, _draw_params: SpriteDrawParams) {
        match world.game_state {
            GameState::StartMenu => {
                // Draw blinking text!
                if self.game_time.fract() < 0.5 {
                    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                                        80, 282.0, 652.0, 900, &self.ui_projection, target);
                    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                                        80, 280.0, 650.0, 900, &self.ui_projection, target);
                }
            }
            GameState::GameOver => {
                // FIXME: Why does this text render weird???
                self.text.draw_text("The Pugs win again... Press Enter to retry.", &self.font, [0.0, 0.0, 0.0],
                                    70, 82.0, 62.0, 900, &self.ui_projection, target);
                self.text.draw_text("The Pugs win again... Press Enter to retry.", &self.font, [1.0, 1.0, 1.0],
                                    70, 80.0, 60.0, 900, &self.ui_projection, target);
            }
            GameState::Won => {
                self.text.draw_text("Mail delivered! Press Enter to proceed!", &self.font, [0.0, 0.0, 0.0],
                                    80, 82.0, 62.0, 900, &self.ui_projection, target);
                self.text.draw_text("Mail delivered! Press Enter to proceed!", &self.font, [1.0, 1.0, 1.0],
                                    80, 80.0, 60.0, 900, &self.ui_projection, target);
            }
            GameState::Credits => {
                let names_margin = 800.0;
                let font_size = 40;
                self.text.draw_text("Justin Hamilton", &self.font, [0.0, 0.0, 0.0],
                                     font_size, names_margin, 400.0, 500, &self.ui_projection, target);
                self.text.draw_text("Gabriel Martinez", &self.font, [0.0, 0.0, 0.0],
                                     font_size, names_margin, 450.0, 500, &self.ui_projection, target);
                self.text.draw_text("Linda Cai", &self.font, [0.0, 0.0, 0.0],
                                     font_size, names_margin, 500.0, 500, &self.ui_projection, target);
                self.text.draw_text("Angelo Yazar", &self.font, [0.0, 0.0, 0.0],
                                     font_size, names_margin, 550.0, 500, &self.ui_projection, target);

                if self.game_time.fract() < 0.5 {
                    self.text.draw_text("Press Enter to restart!", &self.font, [0.0, 0.0, 0.0],
                                        50, 292.0, 702.0, 900, &self.ui_projection, target);
                    self.text.draw_text("Press Enter to restart!", &self.font, [1.0, 1.0, 1.0],
                                        50, 292.0, 700.0, 900, &self.ui_projection, target);
                }
            }
            _ => {}
        }
    }
}

fn load_tiles(tilesets: &[Tileset], midgar: &Midgar) -> Vec<TextureRegion> {
    let mut tiles = Vec::new();

    for tileset in tilesets {
        // TODO: Take into account tile IDs.
        //let mut next_gid = tileset.first_gid;
        for tile in &tileset.tiles {
            let path = format!("assets/tiled/maps/{}", &tile.images[0].source);
            let texture = Rc::new(midgar.graphics().load_texture(path, false));

            // Iterate over tile sizes and create new Tiles.
            let num_cols = tile.images[0].width as u32 / tileset.tile_width;
            let num_rows = tile.images[0].height as u32 / tileset.tile_height;

            // Iterate backwards since Tiled counts tiles from the top left and Midgar draws
            // things from the bottom left.
            for row in 0..num_rows {
                for col in 0..num_cols {
                    // FIXME: Take margin and spacing into account.
                    let offset = (tileset.tile_width * col, tileset.tile_height * row);
                    let size = (tileset.tile_width, tileset.tile_height);
                    let region = TextureRegion::with_sub_field(texture.clone(), offset, size);

                    tiles.push(region);
                }
            }
        }
    }

    tiles
}

fn grid_to_isometric(x: u32, y: u32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let iso_x = (x as i32 - y as i32) as f32 * tile_width / 2.0;
    let iso_y = (x + y) as f32 * tile_height / 2.0;

    (iso_x, iso_y)
}
