// contains the components for players and inventory

use amethyst::ecs::{Component, Entity, VecStorage};
use amethyst::core::math::Vector3;

/// A component that tracks the state of
/// a player.
#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub speed: f32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

#[derive(Debug,Copy,Clone)]
pub struct Position {
    pub pos: Vector3<f32>,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

/// A component that tracks the state of
/// the interface.
#[derive(Debug)]
pub struct Interface {
    pub fps: Entity,
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

//pub struct Inventory {
//    money: u64,
//}

//impl Component for Inventory {
//    type Storage = VecStorage<Self>;
//}

//trait Carryable {
//    fn get_weight(&self) -> f32;
//}
//
//trait Item {
//    fn get_name(&self) -> &str;
//}
