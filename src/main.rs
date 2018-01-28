extern crate cgmath;
extern crate clap;
extern crate midgar;
extern crate rand;
extern crate ears;
extern crate tiled;

mod app;
mod config;
mod renderer;
mod sounds;
mod world;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("Sneky Fox")
        .with_screen_size((config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
