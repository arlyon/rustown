use amethyst::{
    ecs::{Entity, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::ActiveCamera,
    ui::UiText,
    utils::fps_counter::FpsCounter,
};

use crate::components;

#[derive(Debug)]
pub struct Interface {
    pub ui: Entity,
}

/// Handles the UI.
pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = (
        ReadExpect<'a, Interface>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        Read<'a, ActiveCamera>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (interface, mut ui_text, fps_counter, camera, positions): Self::SystemData) {
        let fps = format!("fps: {}", fps_counter.sampled_fps());
        let coords = camera
            .entity
            .and_then(|e| positions.get(e))
            .map(|p| format!("x: {}, y: {}", p.trans.x, p.trans.y))
            .unwrap_or("".to_string());

        if let Some(text) = ui_text.get_mut(interface.ui) {
            text.text = format!("{}\n{}", fps, coords)
        };
    }
}
