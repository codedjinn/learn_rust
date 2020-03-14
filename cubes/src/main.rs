
extern crate amethyst;

mod components;
mod states;
mod systems;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    input::{
        InputBundle,
        StringBindings
    },
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{scene::BasicScenePrefab}
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let display_config = "./config/display.ron";
    let assets_dir = "./assets";

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.38, 0.58, 0.98, 1.0])
                )
                .with_plugin(RenderShaded3D::default())
        )?;

    let mut game = Application::new(assets_dir, states::PlayState, game_data)?;
    game.run();

    Ok(())
}