// bootstraps the game and runs it

mod components;
mod states;
mod systems;
mod util;

use amethyst::{
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");
    let binding_path = resources.join("input_bindings.ron");

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
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PlayerMovementSystem, "movement_system", &[])
        .with(systems::PlayerHealthSystem, "health_system", &[])
        .with(systems::PlayerControlSystem, "control_system", &[])
        .with(systems::UiSystem, "ui_system", &[]);
        ;
    let mut game = Application::new(resources, states::GameplayState, game_data)?;
    game.run();

    Ok(())
}
