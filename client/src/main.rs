//! Bootstraps the game and runs it

pub mod components;
pub mod states;
pub mod systems;
pub mod util;
pub mod sound;
pub mod resource;

use amethyst::{
    audio::{AudioBundle, DjSystem},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    LoggerConfig,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DisplayConfig {
    pub render_distance: u16,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            render_distance: 20
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("config/display.ron");
    let binding_path = resources.join("config/input.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle {})?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::UiSystem, "ui_system", &[])
        .with(DjSystem::new(|music: &mut sound::Music| music.music.next()), "dj_system", &[])
        .with(systems::PlayerMovementSystem, "movement_system", &[])
        .with(systems::PlayerHealthSystem, "health_system", &[])
        .with(systems::PlayerControlSystem, "control_system", &[])
        .with(systems::WorldRenderSystem{zoom: 2.0}, "world_render_system", &[])
        ;
    let mut game = Application::new(resources, states::GameplayState, game_data)?;
    game.run();

    Ok(())
}
