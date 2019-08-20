use crate::components;
use amethyst::ecs::{Join, System, WriteStorage};

/// Handles the health of the actors in the game.
pub struct ActorHealthSystem;
impl<'a> System<'a> for ActorHealthSystem {
    type SystemData = (WriteStorage<'a, components::Actor>);

    fn run(&mut self, mut players: Self::SystemData) {
        for player in (&mut players).join() {
            player.health += 1;
        }
    }
}
