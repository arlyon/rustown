use crate::components;
use amethyst::ecs::Entity;

#[derive(Copy, Clone, Debug)]
pub enum Target {
    Position(components::Position),
    Entity(Entity),
    None,
}
