
use amethyst::{
    prelude::*,
    assets::{
        PrefabLoader,
        PrefabLoaderSystemDesc,
        RonFormat
    },
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord}
    },
    utils::scene::BasicScenePrefab
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

pub struct PlayState;

impl SimpleState for PlayState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        // let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/sphere.ron", RonFormat, ())
        // });
        // data.world.create_entity().with(handle).build();

    }
}