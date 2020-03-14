mod pong;
mod systems;

extern crate amethyst;

use crate::pong::Pong;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    input::{
        InputBundle,
        StringBindings
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let config_dir = "./config/display.ron";
    let binding_dir = "./config/bindings.ron";
    let assets_dir = "./assets";

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_dir)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(config_dir)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with()

    let mut world = World::new();
    let mut game = Application::new(assets_dir, pong::Pong, game_data)?;
    game.run();

    Ok(())
}
