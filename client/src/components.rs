// contains the components for players and inventory

use amethyst::core::math::Vector3;
use amethyst::ecs::{Component, Entity, VecStorage};

/// A component that tracks the state of
/// a player.
#[derive(Debug)]
pub struct Actor {
    pub health: u32,
    pub speed: f32,
}
impl Component for Actor {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub pos: Vector3<f32>,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub mod markers {
    use amethyst::ecs::{Component, VecStorage};

    /// A marker that signifies that the
    /// current entity can be controlled.
    #[derive(Debug)]
    pub struct Controllable;
    impl Component for Controllable {
        type Storage = VecStorage<Self>;
    }
}
