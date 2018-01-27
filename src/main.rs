extern crate cgmath;
extern crate midgar;
extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate ears;

mod app;
mod config;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("ggj-2018")
        .with_screen_size((config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
