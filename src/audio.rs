use std::time::Duration;

use rodio::Source;

use crate::encoder::Encoder;

pub struct AudioSource {
    sounds: BeepVec,
    current_sound: usize,
    sample: u32,
    freq: f32,
    samples_per_symbol: u32,
}

impl AudioSource {
    pub const FRAME_RATE: u32 = 48000;

    pub fn new(freq: f32, samples_per_symbol: u32) -> Self {
        AudioSource {
            freq,
            samples_per_symbol,
            current_sound: 0,
            sample: 0,
            sounds: BeepVec::new(),
        }
    }
}

impl Encoder for AudioSource {
    fn dash(&mut self) {
        self.sounds.push(Beep::On);
        self.sounds.push(Beep::On);
        self.sounds.push(Beep::On);
    }

    fn dot(&mut self) {
        self.sounds.push(Beep::On);
    }

    fn space(&mut self) {
        self.sounds.push(Beep::Off);
    }

    fn pop(&mut self) {
        self.sounds.pop();
    }
}

impl Iterator for AudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sample = (self.sample + 1) % self.samples_per_symbol;
        if self.sample == 0 {
            self.current_sound = self.current_sound.saturating_add(1);
        }

        match self.sounds.get(self.current_sound)? {
            Beep::On => {
                let t = self.sample as f32 / self.samples_per_symbol as f32;
                let value = 2.0 * 3.14159265 * self.freq * t;
                Some(value.sin())
            }
            Beep::Off => Some(0.0),
        }
    }
}

impl Source for AudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        AudioSource::FRAME_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

struct BeepVec {
    data: Vec<u32>,
    last_node_len: usize,
}

#[derive(Debug, Clone, Copy)]
enum Beep {
    Off = 0,
    On = 1,
}

impl BeepVec {
    const NODE_LEN: usize = 32;

    fn new() -> Self {
        BeepVec {
            data: vec![],
            last_node_len: Self::NODE_LEN,
        }
    }

    pub fn push(&mut self, b: Beep) {
        if self.last_node_len >= Self::NODE_LEN {
            self.data.push(0);
            self.last_node_len = 0;
        }

        *self.data.last_mut().unwrap() |= (b as u32) << self.last_node_len;
        self.last_node_len += 1;
    }

    pub fn pop(&mut self) {
        self.last_node_len = self.last_node_len.saturating_sub(1);
        if self.last_node_len == 0 {
            self.pop();
            self.last_node_len = Self::NODE_LEN;
        }
    }

    pub fn get(&self, i: usize) -> Option<Beep> {
        let r = i / Self::NODE_LEN;
        let c = i % Self::NODE_LEN;

        let b = self.data.get(r)?;

        if r == self.data.len() - 1 && c >= self.last_node_len {
            return None;
        }

        let beep = match (b >> c) & 1 {
            0 => Beep::Off,
            1 => Beep::On,
            _ => unreachable!(),
        };

        Some(beep)
    }
}
