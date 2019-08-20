use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use std::default;

use super::util;
use crate::components;

const TILE_SIZE: f32 = 32.0;

/// A target that the given camera should focus on.
pub struct CameraTarget {
    pub camera: Option<Entity>,
    pub target: util::Target,
}

impl default::Default for CameraTarget {
    fn default() -> Self {
        CameraTarget {
            camera: None,
            target: util::Target::None,
        }
    }
}

pub struct WorldGenerationSystem;
impl<'a> System<'a> for WorldGenerationSystem {
    type SystemData = (Read<'a, CameraTarget>,);

    fn run(&mut self, (target): Self::SystemData) {
        let generate_center = match target.target {
            util::Target::Entity(e) => positions.get(e).map(|f| f.pos),
            util::Target::Position(p) => Some(p.pos),
            util::Target::None => None,
        };
    }
}

impl WorldGenerationSystem {}

/// Handles rendering the world.
///
/// todo(arlyon) handle dynamic reframing of "interesting"
///              stuff
pub struct WorldRenderSystem {
    pub zoom: f32,
}
impl<'a> System<'a> for WorldRenderSystem {
    type SystemData = (
        Read<'a, CameraTarget>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, components::Position>,
        Entities<'a>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
        ReadStorage<'a, Camera>,
    );

    fn run(
        &mut self,
        (target, mut transforms, positions, entities, input, time, camera): Self::SystemData,
    ) {
        let camera_position = match target.target {
            util::Target::Entity(e) => positions.get(e).map(|f| f.pos),
            util::Target::Position(p) => Some(p.pos),
            util::Target::None => None,
        };

        if camera_position.is_none() {
            for (ent, _) in (&entities, &positions).join() {
                transforms.remove(ent);
            }
            return;
        }

        self.zoom += input.axis_value("zoom").unwrap() * time.delta_real_seconds();
        let scale = Vector3::new(self.zoom, self.zoom, 1.0);

        for (entity, entity_position) in (&entities, &positions).join() {
            // todo(arlyon) https://github.com/slide-rs/specs/issues/623#issuecomment-522794669
            //              "how to add transforms to all who don't have one"
            let mut transform = transforms.get_mut(entity);
            if transform.is_none() {
                transforms.insert(entity, Transform::default());
                transform = transforms.get_mut(entity);
            }

            let transform = transform.unwrap();
            let delta = (entity_position.pos - camera_position.unwrap()) * self.zoom * TILE_SIZE;
            transform.set_translation(delta);
            transform.set_scale(scale);
        }
    }
}
