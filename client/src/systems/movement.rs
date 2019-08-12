// contains the logic the for entity movement and health systems

extern crate amethyst;

use amethyst::ecs::{Join, System, WriteStorage, ReadStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::core::math::{Vector3,Unit,};
use amethyst::core::num::Zero;

use super::super::components::player::Player;

pub struct PlayerMovementSystem {
    pub target: Transform,
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Player>);

    // all the non-player entities move toward the player
    fn run(&mut self, (mut transforms, players): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let direction: Vector3<f32> = self.target.translation() - transform.translation();
            let direction = if direction.magnitude() < 5.0 { continue } else {Unit::new_normalize(direction)};
            transform.append_translation_along(direction, player.speed);
        }
    }
}

pub struct PlayerHealthSystem;

impl<'a> System<'a> for PlayerHealthSystem {
    type SystemData = (WriteStorage<'a, Player>);

    fn run(&mut self, mut players: Self::SystemData) {
        for player in (&mut players).join() {
            player.health += 1;
        }
    }
}