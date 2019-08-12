// bootstraps the game and runs it

use amethyst::{
    core::transform::TransformBundle,
    core::transform::Transform,
    core::math::Vector3,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod state;

mod components {
    pub mod player;
}

mod systems {
    pub mod movement;
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::movement::PlayerMovementSystem{target: Vector3::new(600., 400., 1.).into()}, "movement_system", &[])
        .with(systems::movement::PlayerHealthSystem, "health_system", &[]);

    let mut game = Application::new(resources, state::GameplayState, game_data)?;
    game.run();

    Ok(())
}
