// handles game state

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::math::Translation2,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Texture,
    },
    window::ScreenDimensions,
};

use log::info;
use super::components::player::Player;

pub struct PausedState;

impl SimpleState for PausedState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Menu opened!")
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Menu closed!")
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the `GameplayState`.
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
    }
}

pub struct GameplayState;

impl SimpleState for GameplayState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        // add components
        world.register::<Player>();

        init_camera(world, &dimensions);

        // Init entities
        let tex = load_sprites(world);
        init_players(world, &dimensions, &tex);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || (is_key_down(&event, VirtualKeyCode::LWin) && is_key_down(&event, VirtualKeyCode::Q)) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState))
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        Trans::None
    }
}

/// Center the camera in the middle of the screen, and let it cover
/// the entire screen
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 100.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/sheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..=2)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_players(world: &mut World, dimensions: &ScreenDimensions, sprites: &[SpriteRender]) {
    for x in (1..1000).step_by(32) {
        for y in (1..1000).step_by(32) {
            let mut transform = Transform::default();
            transform.set_translation_xyz(x as f32, y as f32, 0.0);
            world
                .create_entity()
                .with(sprites[0].clone())
                .with(transform)
                .build();
        }
    }

    use rand_distr::{Normal, Distribution};
    let normal = Normal::new(0.8, 0.1).unwrap();

    for x in -2..2 {
        for y in -2..2 {
            let mut transform = Transform::default();
            transform.set_translation_xyz(dimensions.height() * 0.5 + (x * 20) as f32, dimensions.width() * 0.5 + (y * 20) as f32, 1.0);
            world.create_entity()
                .with(sprites[2].clone())
                .with(transform)
                .with(Player{health: 100, speed: normal.sample(&mut rand::thread_rng())})
                .build();
        }
    }


}
