use amethyst::ecs::{Component, VecStorage};

/// A marker that signifies that the
/// current entity can be controlled.
#[derive(Debug)]
pub struct Controllable;

impl Component for Controllable {
    type Storage = VecStorage<Self>;
}

/// A marker that signifies that the
/// entity is expected to move frequently.
#[derive(Debug)]
pub struct Actor;

impl Component for Actor {
    type Storage = VecStorage<Self>;
}
