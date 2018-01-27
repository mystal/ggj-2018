use midgar::{self, KeyCode, Midgar};

use config;

pub struct GameApp {
}

impl midgar::App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        GameApp {
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }
    }
}
