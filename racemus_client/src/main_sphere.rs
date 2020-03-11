// //! The simplest Amethyst example.
//
// extern crate amethyst;
//
// use amethyst::{
//     GameDataBuilder,
//     StateData,
//     assets::{
//         PrefabLoader,
//         PrefabLoaderSystemDesc,
//         RonFormat
//     },
//     core::transform::TransformBundle,
//     ecs::prelude::WorldExt,
//     prelude::*,
//     input::is_key_down,
//     renderer::{
//         plugins::{
//             RenderShaded3D,
//             RenderToWindow
//         },
//         rendy::mesh::{
//             Normal,
//             Position,
//             TexCoord
//         },
//         types::{
//             DefaultBackend
//         },
//         RenderingBundle
//     },
//     winit::VirtualKeyCode,
//     window::WindowBundle,
//     utils::{
//         scene::BasicScenePrefab
//     },
// };
//
// type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;
//
// struct ExampleState;
//
// impl SimpleState for ExampleState {
//
//     fn on_start(&mut self, data:StateData<'_, GameData<'_, '_>>) {
//
//         let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
//             loader.load("prefab/sphere.ron", RonFormat, ())
//         });
//
//         data.world.create_entity().with(handle).build();
//     }
//
//     fn handle_event(
//         &mut self,
//         _: StateData<'_, GameData<'_, '_>>,
//         event: StateEvent,
//     ) -> SimpleTrans {
//         if let StateEvent::Window(event) = event {
//             if is_key_down(&event, VirtualKeyCode::Escape) {
//                 Trans::Quit
//             } else {
//                 Trans::None
//             }
//         } else {
//             Trans::None
//         }
//     }
//
// }
//
// fn main() -> amethyst::Result<()> {
//     amethyst::start_logger(Default::default());
//
//     let config_dir = "./config/display.ron";
//     let assets_dir = "./assets";
//
//     let game_data = GameDataBuilder::default()
//         .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
//         .with_bundle(TransformBundle::new())?
//         .with_bundle(
//             RenderingBundle::<DefaultBackend>::new()
//                 .with_plugin(
//                     RenderToWindow::from_config_path(config_dir)?
//                         .with_clear([0.39, 0.58, 0.92, 1.0])
//                 )
//                 .with_plugin(RenderShaded3D::default())
//         )?;
//
//     let mut game = Application::new(assets_dir, ExampleState, game_data)?;
//     game.run();
//
//     Ok(())
// }
