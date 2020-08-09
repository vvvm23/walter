use rodio;

#[derive(Clone, Copy)]
pub enum AudioType {
    Audio,
    Music
}

#[derive(Clone)]
pub struct AudioCommand {
    audio_type: AudioType,
    path: String,
    volume: f32,
}

// rodio spawns threads to read from sources and sends output to devices. So we don't really need a
// queue. However, we need multiple sinks to play sounds simultaneously so we will need to manage
// these.

const NB_SINKS: usize = 4;
pub struct AudioSystem {
    music_sink: rodio::Sink,
    sound_sink: Vec<rodio::Sink>,
    decode_queue: Vec<AudioCommand>,
    nb_pending: usize,
    next_sink: usize
}

impl AudioSystem {
    pub fn new() -> Self {
        let device = rodio::default_output_device().unwrap();
        let mut sound_sink = Vec::with_capacity(NB_SINKS);
        for _ in 0..NB_SINKS {
            sound_sink.push(rodio::Sink::new(&device));
        }

        Self {
            music_sink: rodio::Sink::new(&device),
            sound_sink: sound_sink,
            decode_queue: Vec::new(),
            nb_pending: 0,
            next_sink: 0,
        }
    }

    pub fn update(&mut self) {
        if self.nb_pending == 0 {
            return;
        }

        let command = self.decode_queue[0].clone();
        let file = std::fs::File::open(command.path).unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
        self.decode_queue.drain(0..1);

        match command.audio_type {
            AudioType::Audio => {
                self.sound_sink[self.next_sink].append(source);
                self.next_sink += 1;
                if self.next_sink == NB_SINKS {
                    self.next_sink = 0;
                }
            },
            AudioType::Music => {
                self.music_sink.stop();
                self.music_sink.append(source);
                self.music_sink.play();
            },
        };
    }

    pub fn play_sound(&mut self, path: &str) {
        self.decode_queue.push(AudioCommand {
            audio_type: AudioType::Audio,
            path: path.to_string(),
            volume: 0.5,
        });
        self.nb_pending += 1;
    }

    pub fn play_music(&mut self, path: &str) {
        self.decode_queue.push(AudioCommand {
            audio_type: AudioType::Music,
            path: path.to_string(),
            volume: 0.5,
        });
        self.nb_pending += 1;
    }
}
