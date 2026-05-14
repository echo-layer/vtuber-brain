use tracing::info;
use vtuber_commons::assets::personas;

pub struct LoreManager {
    pub active_persona: String,
}

impl LoreManager {
    pub fn new() -> Self {
        info!("Initializing LoreManager with assets from vtuber-commons");
        // Using the SAMPLE persona from vtuber-commons
        let persona_data = personas::SAMPLE;

        Self {
            active_persona: persona_data.to_string(),
        }
    }

    pub fn get_lore(&self) -> &str {
        &self.active_persona
    }
}

impl Default for LoreManager {
    fn default() -> Self {
        Self::new()
    }
}
