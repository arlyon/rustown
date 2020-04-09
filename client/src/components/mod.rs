use std::default;

use amethyst::core::math::Vector3;
use amethyst::ecs::{Component, VecStorage};

pub mod markers;

/// A component that tracks the state of
/// a player.
#[derive(Debug)]
pub struct Living {
    pub health: u32,
    pub speed: f32,
}

impl Component for Living {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub trans: Vector3<f32>,
}

impl default::Default for Position {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Position {
            trans: Vector3::new(x, y, 0.0),
        }
    }
}

#[derive(Debug)]
pub enum TerrainVariant {
    Air,
    Dirt,
    Grass,
    Water,
}

/// A marker that signifies that the
/// entity is part of the terrain.
#[derive(Debug)]
pub struct Terrain {
    pub variant: TerrainVariant,
}

impl Component for Terrain {
    type Storage = VecStorage<Self>;
}

impl default::Default for Terrain {
    fn default() -> Self {
        Terrain {
            variant: TerrainVariant::Air,
        }
    }
}
