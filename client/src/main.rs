//! Bootstraps the game and runs it

use amethyst::{
    audio::AudioBundle,
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

use crate::bundle::GameBundle;
use crate::config::Settings;

mod bundle;
mod components;
mod config;
mod events;
mod resource;
mod sound;
mod states;
mod systems;
mod util;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_path = resources.join("config/display.ron");
    let binding_path = resources.join("config/input.ron");
    let settings_path = resources.join("config/settings.ron");
    let settings = Settings::load(&settings_path)?;

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_path)?.with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(GameBundle)?;

    let mut game: CoreApplication<GameData> =
        ApplicationBuilder::new(resources, states::PlayState::default())?
            .with_resource(settings)
            .build(game_data)?;

    game.run();

    Ok(())
}
