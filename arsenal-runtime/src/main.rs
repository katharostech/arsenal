use std::path::PathBuf;

#[macro_use]
extern crate amethyst;
use amethyst::{
    assets::{
        AssetPrefab, Completion, Handle, Prefab, PrefabData, PrefabError, PrefabLoader,
        PrefabLoaderSystem, ProgressCounter, RonFormat,
    },
    controls::{ControlTagPrefab, FlyControlBundle},
    core::{Transform, TransformBundle},
    input::{is_close_requested, is_key_down},
    ecs::prelude::*,
    prelude::*,
    renderer::{CameraPrefab, DrawShadedSeparate, LightPrefab, VirtualKeyCode},
    utils::application_root_dir
};

use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystem};

#[macro_use]
extern crate serde;

mod types;
use types::RelativeAssetPrefab;

#[derive(Default)]
struct Scene {
    handle: Option<Handle<Prefab<ScenePrefabData>>>,
}

#[derive(PrefabData, Default, Serialize, Deserialize)]
struct ScenePrefabData {
    transform: Option<Transform>,
    gltf: Option<RelativeAssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    camera: Option<CameraPrefab>,
    light: Option<LightPrefab>,
    fly_tag: Option<ControlTagPrefab>,
}

#[derive(Default)]
struct MainSceneState {
    initialized: bool,
    progress: Option<ProgressCounter>,
}

impl SimpleState for MainSceneState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.progress = Some(ProgressCounter::default());

        world.exec(
            |(loader, mut scene): (PrefabLoader<'_, ScenePrefabData>, Write<'_, Scene>)| {
                scene.handle = Some(loader.load(
                    std::env::args().nth(1).unwrap(),
                    RonFormat,
                    (),
                    self.progress.as_mut().unwrap(),
                ));
            },
        );
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(event) || is_key_down(event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }

        Trans::None              
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if !self.initialized {
            let done_loading = {
                match self.progress.as_ref().map(|p| p.complete()) {
                    None | Some(Completion::Loading) => false,
                    Some(Completion::Complete) => {
                        let scene_handle = data
                            .world
                            .read_resource::<Scene>()
                            .handle
                            .as_ref()
                            .unwrap()
                            .clone();

                        data.world.create_entity().with(scene_handle).build();

                        true
                    }
                    Some(Completion::Failed) => {
                        println!("Error: {:?}", self.progress.as_ref().unwrap().errors());
                        return Trans::Quit;
                    }
                }
            };

            if done_loading {
                self.progress = None;
            }
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = PathBuf::from(application_root_dir());

    let display_config_path = "display_config.ron";

    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<ScenePrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["scene_loader"],
        )
        .with_bundle(
            FlyControlBundle::<String, String>::new(None, None, None)
                .with_sensitivity(0.1, 0.1)
                .with_speed(5.),
        )?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_basic_renderer(display_config_path, DrawShadedSeparate::new(), false)?;

    let mut game = Application::build(app_root, MainSceneState::default())?.build(game_data)?;
    game.run();

    Ok(())
}
