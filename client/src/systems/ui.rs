use amethyst::{
    ecs::{Entity, Join, Read, ReadExpect, System, WriteStorage},
    ui::UiText,
    utils::fps_counter::FpsCounter,
};

#[derive(Debug)]
pub struct Interface {
    pub fps_counter: Entity,
}

/// Handles the UI.
pub struct UiSystem;
impl<'a> System<'a> for UiSystem {
    type SystemData = (
        ReadExpect<'a, Interface>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
    );

    fn run(&mut self, (interface, mut ui_text, fps_counter): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(interface.fps_counter) {
            text.text = format!("fps: {}", fps_counter.sampled_fps());
        }
    }
}
