use clap::{Arg, App};
use midgar::{self, KeyCode, Midgar};

use config;
use renderer::GameRenderer;
use world::GameWorld;
use sounds::{Sounds, AudioController};

pub struct GameApp<'a> {
    world: GameWorld,
    renderer: GameRenderer<'a>,
    sounds: Sounds,
}

impl<'a> midgar::App for GameApp<'a> {
    fn create(midgar: &Midgar) -> Self {
        // Parse args.
        let matches = App::new("Sneky Fox")
            .arg(Arg::with_name("LEVEL")
                 .index(1))
            .get_matches();

        let level = matches.value_of("LEVEL").unwrap_or(config::START_LEVEL.into());
        let world = GameWorld::new(&level, config::ASSETS_PATH);
        let renderer = GameRenderer::new(midgar, &world.level.map.tilesets);
        let sounds = Sounds::new();

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
