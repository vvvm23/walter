use crate::component::audio;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AudioAtlas {
    lookup_audio: HashMap<String, Arc<RwLock<ggez::audio::Source>>>,
}

impl AudioAtlas {
    pub fn new() -> AudioAtlas {
        AudioAtlas {
            lookup_audio: HashMap::new(),
        }
    }

    pub fn load(&mut self, ctx: &mut ggez::Context, path: &str) {
        self.lookup_audio.insert(
            path.to_string(),
            Arc::new(RwLock::new(ggez::audio::Source::new(ctx, path).unwrap()))
        );
    }

    pub fn get(&self, path: &str) -> Arc<RwLock<ggez::audio::Source>> {
        let path = path.to_string();
        assert!(self.lookup_audio.contains_key(&path), "Requested audio file has not been loaded!");
        Arc::clone(self.lookup_audio.get(&path).unwrap())
    }
}
