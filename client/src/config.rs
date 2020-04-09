use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub render_distance: u16,
    pub seed: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            render_distance: 20,
            seed: "deadbeef".to_string(),
        }
    }
}
