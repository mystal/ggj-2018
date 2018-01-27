use midgar::{self, KeyCode, Midgar};

use config;
use renderer::GameRenderer;
use world::GameWorld;

pub struct GameApp {
    world: GameWorld,
    renderer: GameRenderer,
}

impl midgar::App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        let world = GameWorld::new("mockup", config::ASSETS_PATH);
        let renderer = GameRenderer::new(midgar, &world.level.map.tilesets);

        GameApp {
            world,
            renderer,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world);
    }
}
