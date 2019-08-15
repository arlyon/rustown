use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Camera, Texture,
    },
    ui,
    window::ScreenDimensions,
};

use crate::components;

/// Center the camera in the middle of the screen, and let it cover
/// the entire screen
pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 100.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

pub fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
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

pub fn init_players(
    world: &mut World,
    dimensions: &ScreenDimensions,
    sprites: &[SpriteRender],
) -> amethyst::ecs::Entity {
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

    use rand_distr::{Distribution, Normal};
    let normal = Normal::new(0.8, 0.1).unwrap();

    for x in -2..2 {
        for y in -2..2 {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                dimensions.height() * 0.5 + (x * 20) as f32,
                dimensions.width() * 0.5 + (y * 20) as f32,
                1.0,
            );
            world
                .create_entity()
                .with(sprites[2].clone())
                .with(transform)
                .with(components::Player {
                    health: 100,
                    speed: normal.sample(&mut rand::thread_rng()),
                })
                .build();
        }
    }

    let mut transform = Transform::default();
    transform.set_translation_xyz(200.0, 200.0, 1.0);
    world
        .create_entity()
        .with(sprites[1].clone())
        .with(transform)
        .with(components::Player {
            health: 100,
            speed: 2.0,
        })
        .with(components::markers::Controllable)
        .build()
}

pub fn init_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/Gaegu-Regular.ttf",
        ui::TtfFormat,
        (),
        &world.read_resource(),
    );

    let fps_transform = ui::UiTransform::new(
        "fps".to_string(),
        ui::Anchor::TopLeft,
        ui::Anchor::TopLeft,
        10.0,
        -5.0,
        0.0,
        200.0,
        50.0,
    );

    let mut ui_text = ui::UiText::new(
        font.clone(),
        "framerate".to_string(),
        [1.0, 1.0, 1.0, 1.0],
        30.0,
    );

    ui_text.align = ui::Anchor::TopLeft;

    let fps = world
        .create_entity()
        .with(fps_transform)
        .with(ui_text)
        .build();

    world.add_resource(components::Interface { fps })
}
