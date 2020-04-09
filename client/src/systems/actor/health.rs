use amethyst::ecs::{Join, System, WriteStorage};

use crate::components;

/// Handles the health of the actors in the game.
pub struct ActorHealthSystem;

impl<'a> System<'a> for ActorHealthSystem {
    type SystemData = WriteStorage<'a, components::Living>;

    fn run(&mut self, mut players: Self::SystemData) {
        for player in (&mut players).join() {
            player.health += 1;
        }
    }
}
