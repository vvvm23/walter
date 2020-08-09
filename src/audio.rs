pub struct AudioCommand {
    path: String,
    volume: f32,
}

pub struct AudioSystem {
    pending: Vec<AudioCommand>,
    nb_pending: u32,
    max_pending: u32,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            nb_pending: 0,
            max_pending: 32,
        }
    }

    pub fn play_sound(&mut self) {
        
    }

    pub fn update(&mut self) {

    }
}
