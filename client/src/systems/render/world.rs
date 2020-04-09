//!
//! Renders the game world.
//!
//! The game world is immobile,
//! and is only re-rendered occasionally.
//!

use amethyst::prelude::{World, WorldExt};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        ActiveCamera, Texture,
    },
};
use shrev::{EventChannel, ReaderId};

use crate::components::{Position, Terrain, TerrainVariant};
use crate::events::WorldEvent;
use crate::systems::render::common::TILE_SIZE;

/// Handles rendering the world.
pub struct WorldRenderSystem {
    reader_id: ReaderId<WorldEvent>,
    sprites: Vec<SpriteRender>,
}

impl WorldRenderSystem {
    pub fn new(world: &mut World) -> Self {
        let reader_id = world
            .fetch_mut::<EventChannel<WorldEvent>>()
            .register_reader();
        Self {
            reader_id,
            sprites: vec![],
        }
    }
}

impl<'a> System<'a> for WorldRenderSystem {
    type SystemData = (
        Read<'a, ActiveCamera>,
        Entities<'a>,
        ReadStorage<'a, Terrain>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Transform>,
        Read<'a, EventChannel<WorldEvent>>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(
        &mut self,
        (camera, entities, terrain, positions, mut transforms, events, mut sprites): Self::SystemData,
    ) {
        let should_update = events.read(&mut self.reader_id).any(|e| match e {
            WorldEvent::Updated => true,
            _ => false,
        });

        if !should_update {
            return;
        } else {
            log::debug!("drawing world")
        }

        let (cam_pos, cam_trans) =
            match camera.entity.map(|e| (positions.get(e), transforms.get(e))) {
                Some((Some(p), Some(t))) => (p, t.translation().clone()),
                _ => return,
            };

        let missing_transforms: Vec<_> = (&entities, &terrain, !&transforms)
            .join()
            .map(|(e, _, _)| e)
            .collect();

        for e in missing_transforms {
            transforms.insert(e, Transform::default());
        }

        let missing_sprites: Vec<_> = (&entities, &terrain, !&sprites)
            .join()
            .map(|(e, _, _)| e)
            .collect();

        for e in missing_sprites {
            sprites.insert(e, self.sprites[4].clone());
        }

        for (terrain, pos, trans, sprite) in
            (&terrain, &positions, &mut transforms, &mut sprites).join()
        {
            let mut t = cam_trans + (pos.trans - cam_pos.trans) * TILE_SIZE;
            t.z = 0.0;
            log::debug!("{} {} {}", t.x, t.y, t.z);
            trans.set_translation(t);
            sprite.sprite_number = match terrain.variant {
                TerrainVariant::Air => 5,
                TerrainVariant::Grass => 0,
                TerrainVariant::Dirt => 3,
                TerrainVariant::Water => 4,
            };
        }
    }

    // todo(arlyon) duplicate spritesheet init
    fn setup(&mut self, world: &mut World) {
        let loader = world.read_resource::<Loader>();

        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/sheet.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let sheet_handle = {
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
        self.sprites = (0..=5)
            .map(|i| SpriteRender {
                sprite_sheet: sheet_handle.clone(),
                sprite_number: i,
            })
            .collect();
    }
}
