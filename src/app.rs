use midgar::{self, KeyCode, Midgar};

use config;
use renderer::GameRenderer;
use world::GameWorld;
use sounds::{Sounds, AudioController};

pub struct GameApp {
    world: GameWorld,
    renderer: GameRenderer,
    sounds: Sounds,
}

impl midgar::App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        let world = GameWorld::new("level_4", config::ASSETS_PATH);
        let renderer = GameRenderer::new(midgar, &world.level.map.tilesets);
        let mut sounds = Sounds::new();

        GameApp {
            world,
            renderer,
            sounds,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        if !self.sounds.background_music.is_playing() {
            self.sounds.background_music.set_volume(0.1);
            self.sounds.background_music.play();
        }

        let dt = midgar.time().delta_time() as f32;

        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world);
    }
}
