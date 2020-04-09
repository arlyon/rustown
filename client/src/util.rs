use amethyst::{
    assets::{AssetStorage, Loader},
    core::math::Vector3,
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Camera, Texture,
    },
    ui,
};

use crate::components;
use crate::components::Position;
use crate::systems;

/// Center the camera in the middle of the screen, and let it cover
/// the entire screen
pub fn init_camera(world: &mut World, width: f32, height: f32) -> Entity {
    world
        .create_entity()
        .with(Position {
            trans: Vector3::new(0.0, 0.0, 10.0),
        })
        .with(Camera::standard_2d(width, height))
        .with(Transform::from(Vector3::new(0.0, 0.0, 10.0)))
        .build()
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
    (0..=3)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

pub fn init_players(world: &mut World, sprites: &[SpriteRender]) -> amethyst::ecs::Entity {
    world
        .create_entity()
        .with(Transform::default())
        .with(sprites[1].clone())
        .with(components::Position::new(0.0, 0.0))
        .with(components::Living {
            health: 100,
            speed: 2.0,
        })
        .with(components::markers::Actor)
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

    let ui_transform = ui::UiTransform::new(
        "".to_string(),
        ui::Anchor::TopLeft,
        ui::Anchor::TopLeft,
        10.0,
        -5.0,
        0.0,
        500.0,
        200.0,
    );

    let mut ui_text = ui::UiText::new(font, "framerate".to_string(), [1.0, 1.0, 1.0, 1.0], 30.0);
    ui_text.align = ui::Anchor::TopLeft;
    ui_text.line_mode = ui::LineMode::Wrap;

    let ui = world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build();

    world.insert(systems::Interface { ui })
}
