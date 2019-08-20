use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use std::default;

use super::util;
use crate::components;

/// A target to move to.
#[derive(Debug)]
pub struct ActorAITarget {
    pub target: util::Target,
}

impl default::Default for ActorAITarget {
    fn default() -> Self {
        ActorAITarget {
            target: util::Target::None,
        }
    }
}

/// Handles the movement of the actors in the game.
pub struct ActorMovementSystem;
impl<'a> System<'a> for ActorMovementSystem {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Actor>,
        Read<'a, ActorAITarget>,
        Read<'a, Time>,
    );

    // all the non-player entities move toward the player.
    fn run(&mut self, (mut positions, players, target, time): Self::SystemData) {
        let target_transform = match &target.target {
            util::Target::Position(p) => Some(p.pos),
            util::Target::Entity(e) => positions.get(*e).map(|p| p.pos),
            util::Target::None => None,
        };

        let target_transform = if let Some(transform) = target_transform {
            transform.to_owned()
        } else {
            return;
        };

        for (player, player_position) in (&players, &mut positions).join() {
            let direction: Vector3<f32> = target_transform - player_position.pos;
            if direction.magnitude() < 0.1 {
                continue;
            };
            player_position.pos += direction.normalize() * player.speed * time.delta_real_seconds();
        }
    }
}
