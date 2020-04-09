use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components;

/// Handles the controls for the players in the game.
pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        ReadStorage<'a, components::Living>,
        ReadStorage<'a, components::markers::Controllable>,
        WriteStorage<'a, components::Position>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (players, controllables, mut positions, input, time): Self::SystemData) {
        for (player, _, position) in (&players, &controllables, &mut positions).join() {
            let direction = Vector3::new(
                -input.axis_value("horizontal").unwrap(),
                input.axis_value("vertical").unwrap(),
                0.0,
            );

            if direction.magnitude() < 0.01 {
                continue;
            };

            position.trans += direction.normalize() * player.speed * time.delta_real_seconds();
        }
    }
}
