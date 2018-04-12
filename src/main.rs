extern crate ggez;

use ggez::{ContextBuilder, GameResult};
use ggez::conf;
use ggez::Context;
use ggez::event;
use ggez::event::EventHandler;
use ggez::error::GameError;
use ggez::conf::Backend;

use std::env;
use std::path;
use ggez::graphics;

enum Scene {
    Intro,
    Game,
}

struct MainState {
    scene: Scene,
}

impl MainState {
    const DESIRED_FPS: u64 = 60;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        let s = MainState{
            scene: Scene::Intro
        };

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.scene {
            Scene::Game => {

            }
            _ => {}
        }
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.scene {
            Scene::Game => {

            }
            Scene::Intro => {

            }
        }
        Ok(())
    }
}

fn main() {
    let mut cb = ContextBuilder::new("Chemistry-panic", "Kimond")
        .window_setup(conf::WindowSetup::default().title("Chemistry panic"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();
    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }

        }
    }
}
