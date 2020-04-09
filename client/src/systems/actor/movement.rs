use std::default;

use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components;
use crate::systems::util;

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
        ReadStorage<'a, components::Living>,
        Read<'a, ActorAITarget>,
        Read<'a, Time>,
    );

    // all the non-player entities move toward the player.
    fn run(&mut self, (mut positions, players, target, time): Self::SystemData) {
        let target_transform = match &target.target {
            util::Target::Position(p) => Some(p.trans),
            util::Target::Entity(e, _) => positions.get(*e).map(|p| p.trans),
            util::Target::None => None,
        };

        let target_transform = if let Some(transform) = target_transform {
            transform.to_owned()
        } else {
            return;
        };

        for (player, player_position) in (&players, &mut positions).join() {
            let direction: Vector3<f32> = target_transform - player_position.trans;
            if direction.magnitude() < 0.1 {
                continue;
            };
            player_position.trans +=
                direction.normalize() * player.speed * time.delta_real_seconds();
        }
    }
}
