use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> Option<Self> {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                return Some(AudioPlayer { _stream, sink });
            }
        }
        None
    }

    fn get_audio_path(filename: &str) -> PathBuf {
        PathBuf::from("audio").join(filename)
    }

    fn play_sound(&self, filename: &str) {
        let path = Self::get_audio_path(filename);
        if let Ok(file) = File::open(&path) {
            let source = BufReader::new(file);
            if let Ok(decoder) = Decoder::new(source) {
                self.sink.append(decoder);
            }
        }
    }

    pub fn play_bat_contact(&self) {
        self.play_sound("bat.wav");
    }

    pub fn play_catch(&self) {
        self.play_sound("catch.wav");
    }

    pub fn play_ground_ball(&self) {
        let filename = if rand::random() { "ground_1.wav" } else { "ground_2.wav" };
        self.play_sound(filename);
    }

    pub fn play_miss(&self) {
        let choice = rand::random::<usize>() % 3 + 1;
        let filename = format!("miss_{}.wav", choice);
        self.play_sound(&filename);
    }

    pub fn play_cheer_single(&self) {
        self.play_sound("cheer_single.wav");
    }

    pub fn play_cheer_double(&self) {
        self.play_sound("cheer_double.wav");
    }

    pub fn play_cheer_triple_and_homer(&self) {
        self.play_sound("cheer_triple_and_homer.wav");
    }
}
