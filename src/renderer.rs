use std::rc::Rc;

use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SamplerWrapFunction, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;

use config;
use world::*;

pub struct GameRenderer {
    projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    text: TextRenderer,

    background: TextureRegion,
    tile_grass: TextureRegion,
    tile_dirt: TextureRegion,
    sneky_fox: TextureRegion,
    sneky_fox_with_mail: TextureRegion,
    mailbox: TextureRegion,
    letter_1: TextureRegion,
    letter_2: TextureRegion,
    pug: TextureRegion,

    game_time: f32,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        // Load textures.
        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/background.png", false));
            TextureRegion::new(texture)
        };
        let tile_grass = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/tile_grass.png", false));
            TextureRegion::new(texture)
        };
        let tile_dirt = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/tile_dirt.png", false));
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

        // TODO: For when we have a camera?
        let projection = cgmath::ortho(-(config::GAME_SIZE.x as f32 / 2.0), config::GAME_SIZE.x as f32 / 2.0,
                                       config::GAME_SIZE.y as f32 / 2.0, -(config::GAME_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);
        //let projection = cgmath::ortho(0.0, config::GAME_SIZE.x as f32,
        //                               config::GAME_SIZE.y as f32, 0.0,
        //                               -1.0, 1.0);

        GameRenderer {
            projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
            text: TextRenderer::new(midgar.graphics().display()),

            background,
            tile_grass,
            tile_dirt,
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
        self.sprite.set_projection_matrix(self.projection);
        self.shape.set_projection_matrix(self.projection);

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        // Draw background.
        self.sprite.draw(&self.background.draw(0.0, 0.0), draw_params, target);

        // Draw tiles.
        let color = [1.0, 1.0, 1.0];
        let width = 32.0;
        let height = 32.0;
        for (tile, x, y) in world.level.iter_tiles() {
            match tile {
                Tile::Floor => {
                    self.shape.draw_filled_rect(x as f32 * width, y as f32 * height,
                                                width, height, color, target);
                }
                _ => {}
            }
        }

        // Draw mailbox.
        self.shape.draw_filled_rect(world.mailbox.pos.x as f32 * width, world.mailbox.pos.y as f32 * height,
                                    16.0, 16.0, [0.0, 0.8, 0.3], target);

        // Draw fox.
        self.shape.draw_filled_rect(world.fox.pos.x as f32 * width, world.fox.pos.y as f32 * height,
                                    20.0, 20.0, [1.0, 0.25, 0.0], target);
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
