use std::default;

use amethyst::ecs::Entity;

use crate::components;

#[derive(Copy, Clone, Debug)]
pub enum Target {
    Position(components::Position),
    Entity(Entity, f32),
    None,
}

/// A target that the given camera should follow.
pub struct CameraTarget {
    pub camera: Option<Entity>,
    pub target: Target,
}

impl default::Default for CameraTarget {
    fn default() -> Self {
        CameraTarget {
            camera: None,
            target: Target::None,
        }
    }
}
