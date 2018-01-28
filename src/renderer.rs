use std::rc::Rc;

use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SamplerWrapFunction, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;
use tiled::Tileset;

use config;
use world::*;

pub struct GameRenderer {
    projection: Matrix4<f32>,
    ui_projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    text: TextRenderer,

    tiles: Vec<TextureRegion>,
    background: TextureRegion,
    sneky_fox: TextureRegion,
    sneky_fox_with_mail: TextureRegion,
    mailbox: TextureRegion,
    letter_1: TextureRegion,
    letter_2: TextureRegion,
    pug: TextureRegion,

    game_time: f32,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar, tilesets: &Vec<Tileset>) -> Self {
        // Load textures.
        let tiles = load_tiles(tilesets, midgar);
        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/background.png", false));
            TextureRegion::new(texture)
        };
        let sneky_fox = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_fox.png", false));
            TextureRegion::new(texture)
        };
        let sneky_fox_with_mail = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/sneky_fox_with_mail.png", false));
            TextureRegion::new(texture)
        };
        let mailbox = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/mailbox.png", false));
            TextureRegion::new(texture)
        };
        let letter_1 = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/letter_1.png", false));
            TextureRegion::new(texture)
        };
        let letter_2 = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/letter_2.png", false));
            TextureRegion::new(texture)
        };
        let pug = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/pug.png", false));
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
            sneky_fox,
            sneky_fox_with_mail,
            mailbox,
            letter_1,
            letter_2,
            pug,

            game_time: 0.0,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        self.game_time += dt;

        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        match world.game_state {
            GameState::Running => {
                self.draw_world(dt, world, &mut target);
                self.draw_ui(dt, world, &mut target);
            }
            _ => {
                self.draw_world(dt, world, &mut target);
                self.draw_ui(dt, world, &mut target);
            }
        }

        target.finish().unwrap();
    }

    fn draw_world<S: Surface>(&mut self, dt: f32, world: &GameWorld, target: &mut S) {
        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

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
        // TODO: Figure out object offsets so they sit on tiles correctly.

        // Draw mailbox.
        let pos = world.mailbox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        self.sprite.draw(&self.mailbox.draw(draw_x, draw_y),
                         draw_params, target);

        // Draw mail
        if !world.fox.has_mail {
            let pos = world.mail.pos;
            let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
            self.sprite.draw(&self.letter_1.draw(draw_x, draw_y),
                             draw_params, target);
        }

        // Draw fox.
        let pos = world.fox.pos;
        let (draw_x, draw_y) = grid_to_isometric(pos.x, pos.y, tile_width, tile_height);
        self.sprite.draw(&self.sneky_fox.draw(draw_x, draw_y),
                         draw_params, target);
    }

    fn draw_ui<S: Surface>(&mut self, _dt: f32, world: &GameWorld, target: &mut S) {
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
