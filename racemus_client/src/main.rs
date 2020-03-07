//! Opens an empty window.

extern crate amethyst;

use amethyst::prelude::*;
use amethyst::window::DisplayConfig;
use amethyst::renderer::plugins::RenderFlat2D;

use amethyst::utils::application_root_dir;

struct Pong;

impl SimpleState for Pong {
}

const APP_ROOT : &str = "c:\\dev\\learn_rust\\racemus_client";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // need to figure out why methods are prepending \\?
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config\\display.ron");
    let assets_dir = app_root.join("assets\\");

    //let path = format!("{}/config/display.ron", application_root_dir());
    let config = DisplayConfig::load("c:\\dev\learn_rust\\racemus_client\\src\\config\\display.ron");

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
            .clear_target([0.39, 0.58, 0.92, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe ,Some(config))
                .with_sprite_sheet_processor()
        )?;

    let mut game = Application::new("./", Pong, game_data)?;
    game.run();
    // let app_root = String::from("c:\\dev\\learn_rust\\racemus_client\\target\\debug");
    // let display_config_path = "c:\\dev\\learn_rust\\racemus_client\\target\\debug\\config\\display.ron";
    // let assets_dir = "c:\\dev\\learn_rust\\racemus_client\\target\\debug\\";

    // let game_data = GameDataBuilder::default()
    //     .with_bundle(WindowBundle::from_config_path(display_config_path)?)?;

    // let mut game = Application::new(assets_dir, SimpleState, game_data)?;
    // game.run();

    Ok(())
}
