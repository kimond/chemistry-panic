extern crate ggez;

use assets::Assets;
use ggez::{ContextBuilder, GameResult};
use ggez::conf;
use ggez::conf::Backend;
use ggez::Context;
use ggez::error::GameError;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use std::env;
use std::path;

mod assets;

enum Scene {
    Intro,
    Game,
}

struct MainState {
    scene: Scene,
    assets: Assets,
    window_size: (u32, u32),
}

impl MainState {
    const DESIRED_FPS: u64 = 60;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let window_size = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 0).into());
        let assets = Assets::new(ctx)?;

        let s = MainState {
            scene: Scene::Intro,
            assets,
            window_size,
        };

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.scene {
            Scene::Game => {}
            _ => {}
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx);
        match self.scene {
            Scene::Game => {
                let player_img = graphics::Image::new(ctx, "/player_01.png")?;
                let player_dest = graphics::Point2::new(self.window_size.0 as f32 / 2.0 - 30.0, self.window_size.1 as f32 / 2.0);
                graphics::draw(ctx, &player_img, player_dest, 0.0);

            }
            Scene::Intro => {
                let title_dest = graphics::Point2::new(self.window_size.0 as f32 / 2.0 - 30.0, self.window_size.1 as f32 / 2.0);
                let intro_text_dest = graphics::Point2::new(self.window_size.0 as f32 / 2.0, self.window_size.1 as f32 / 2.0 + 50.0);
                graphics::draw(ctx, &self.assets.title, title_dest, 0.0);
                graphics::draw(ctx, &self.assets.intro_text, intro_text_dest, 0.0);
                graphics::circle(
                    ctx,
                    graphics::DrawMode::Fill,
                    graphics::Point2::new(10.0, 10.0),
                    100.0,
                    2.0,
                ).unwrap()
            }
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match self.scene {
            Scene::Game => {

            }
            _ => if keycode == event::Keycode::Space {
                self.scene = Scene::Game;
            }
        }
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
