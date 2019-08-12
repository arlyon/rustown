// contains the components for players and inventory

use amethyst::ecs::{Component,VecStorage};

#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub speed: f32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

pub struct Inventory {
    money: u64,
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}

trait Carryable {
    fn get_weight(&self) -> f32;
}

trait Item {
    fn get_name(&self) -> &str;
}

