use std::rc::Rc;

use cgmath::{self, Matrix4, Vector2};
use cgmath::prelude::*;
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SamplerWrapFunction, Sprite, SpriteDrawParams, SpriteRenderer};
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
    sneky_fox: Sprite<'a>,
    sneky_fox_with_mail: Sprite<'a>,
    mailbox: Sprite<'a>,
    letter_1: Sprite<'a>,
    letter_2: Sprite<'a>,
    bone: Sprite<'a>,
    pug: Sprite<'a>,

    game_time: f32,
}

impl<'a> GameRenderer<'a> {
    pub fn new(midgar: &Midgar, tilesets: &Vec<Tileset>) -> Self {
        // Load textures.
        let tiles = load_tiles(tilesets, midgar);
        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/background.png", false));
            TextureRegion::new(texture)
        };
        let sneky_fox = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_fox.png", false));
            let mut sprite = Sprite::new(texture);
            sprite.set_scale(Vector2::new(0.8, 0.8));
            let size = sprite.size();
            sprite.set_origin(Vector2::new(92.0 / size.x as f32, 80.0 / size.y as f32));
            sprite
        };
        let sneky_fox_with_mail = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_fox_with_mail.png", false));
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
        let letter_2 = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/letter_2.png", false));
            let mut sprite = Sprite::new(texture);
            // TODO: If we use this, set its origin to be drawn nicely on the grid.
            sprite
        };
        let bone = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/temp_bone.png", false));
            let mut sprite = Sprite::new(texture);
            let size = sprite.size();
            sprite
        };
        let pug = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/pug.png", false));
            let mut sprite = Sprite::new(texture);
            let size = sprite.size();
            sprite.set_origin(Vector2::new(62.0 / size.x as f32, 112.0 / size.y as f32));
            sprite
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
            sneky_fox,
            sneky_fox_with_mail,
            mailbox,
            letter_1,
            letter_2,
            bone,
            pug,

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
            GameState::Running => {
                self.draw_world(dt, world, &mut target, draw_params);
                self.draw_ui(dt, world, &mut target, draw_params);
            }
            GameState::GameOver => {
                self.draw_over(dt, world, &mut target, draw_params);
                self.draw_ui(dt, world, &mut target, draw_params);
            }
            _ => {
                self.draw_world(dt, world, &mut target, draw_params);
                self.draw_ui(dt, world, &mut target, draw_params);
            }
        }

        target.finish().unwrap();
    }

    fn draw_over<S: Surface>(&mut self, dt: f32, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        // Draw background.
        self.sprite.set_projection_matrix(self.ui_projection);
        self.sprite.draw(&self.background.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);

        self.sprite.set_projection_matrix(self.projection);
        self.shape.set_projection_matrix(self.projection);

        // Draw tiles.
        // TODO: Get tile width and height from the Tiled map?
        let tile_width = 180.0;
        let tile_height = 90.0;
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

        // TODO: Draw game objects top-down, left-right in the iso view.
        self.draw_pugs(world, tile_width, tile_height, target, draw_params);
        self.draw_mailbox(world, tile_width, tile_height, target, draw_params);
        self.draw_bones(world, tile_width, tile_height, target, draw_params);
        self.draw_mail(world, tile_width, tile_height, target, draw_params);
        self.draw_dead_fox(world, tile_width, tile_height, target, draw_params);
    }

    fn draw_world<S: Surface>(&mut self, dt: f32, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        // Draw background.
        self.sprite.set_projection_matrix(self.ui_projection);
        self.sprite.draw(&self.background.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                         draw_params, target);

        self.sprite.set_projection_matrix(self.projection);
        self.shape.set_projection_matrix(self.projection);

        // Draw tiles.
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

        // TODO: Draw game objects top-down, left-right in the iso view.
        self.draw_pugs(world, tile_width, tile_height, target, draw_params);
        self.draw_mailbox(world, tile_width, tile_height, target, draw_params);
        self.draw_bones(world, tile_width, tile_height, target, draw_params);
        self.draw_mail(world, tile_width, tile_height, target, draw_params);
        self.draw_fox(world, tile_width, tile_height, target, draw_params);
    }

    fn draw_dead_fox<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        let texture = &mut self.sneky_fox;
        let pos = world.fox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        // NOTE: Subtract 8 pixels to align to the center of the squares.
        texture.set_position(Vector2::new(draw_x, draw_y - 8.0 + world.time * config::FALL_SPEED));
        self.sprite.draw(texture, draw_params, target);
    }


    fn draw_fox<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        let sprite = if world.fox.has_mail {
            &mut self.sneky_fox_with_mail
        } else {
            &mut self.sneky_fox
        };
        let flip_x = world.fox.dir == Direction::East || world.fox.dir == Direction::North;
        sprite.set_flip_x(flip_x);
        let pos = world.fox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        // NOTE: Subtract 8 pixels to align to the center of the squares.
        sprite.set_position(Vector2::new(draw_x, draw_y - 8.0));
        self.sprite.draw(sprite, draw_params, target);
    }

    fn draw_mail<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        if !world.fox.has_mail {
            let pos = world.mail.pos;
            let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
            // NOTE: Subtract 8 pixels to align to the center of the squares.
            self.letter_1.set_position(Vector2::new(draw_x, draw_y - 8.0));
            self.sprite.draw(&self.letter_1, draw_params, target);
        }
    }

    fn draw_bones<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        for bone in &world.bones {
            if bone.is_held() {
                // Draw locations where fox can throw bone
                let v = bone.get_throwable_positions(&world.level);
                for pos in &v {
                    let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
                    self.bone.set_position(Vector2::new(draw_x, draw_y - 8.0));
                    self.sprite.draw(&self.bone, draw_params, target);
                }
            } else {
                // Draw bone
                let pos = bone.pos;
                let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
                // NOTE: Subtract 8 pixels to align to the center of the squares.
                self.bone.set_position(Vector2::new(draw_x, draw_y - 8.0));
                self.sprite.draw(&self.bone, draw_params, target);
            }

        }
    }

    fn draw_mailbox<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        let pos = world.mailbox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        // NOTE: Subtract 8 pixels to align to the center of the squares.
        self.mailbox.set_position(Vector2::new(draw_x, draw_y - 8.0));
        self.sprite.draw(&self.mailbox, draw_params, target);
    }

    fn draw_pugs<S: Surface>(&mut self, world: &GameWorld, tile_width: f32, tile_height: f32, target: &mut S, draw_params: SpriteDrawParams) {
        for pug in &world.pugs {
            let flip_x = pug.dir == Direction::East || pug.dir == Direction::North;
            self.pug.set_flip_x(flip_x);
            let pos = pug.pos;
            let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
            // NOTE: Subtract 8 pixels to align to the center of the squares.
            self.pug.set_position(Vector2::new(draw_x, draw_y - 8.0));
            self.sprite.draw(&self.pug, draw_params, target);
        }
    }

    fn draw_ui<S: Surface>(&mut self, _dt: f32, world: &GameWorld, target: &mut S, draw_params: SpriteDrawParams) {
        let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                       config::SCREEN_SIZE.y as f32, 0.0,
                                       -1.0, 1.0);
        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);
    }
}

fn load_tiles(tilesets: &Vec<Tileset>, midgar: &Midgar) -> Vec<TextureRegion> {
    let mut tiles = Vec::new();

    for tileset in tilesets {
        // TODO: Take into account tile IDs.
        //let mut next_gid = tileset.first_gid;
        for tile in &tileset.tiles {
            let path = format!("assets/tiled/maps/") + &tile.images[0].source;
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
